//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1185/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1185(t11204: f64, t11211: f64, t14702: f64, t14868: f64, t14870: f64, t18742: f64, t18747: f64, t18749: f64, t18752: f64, t18755: f64, t18757: f64, t11137: f64, t14818: f64, t18227: f64, t18239: f64, t18497: f64, t18500: f64, t18503: f64, t18508: f64, t18510: f64, t18515: f64, t18518: f64) -> (f64, f64) {
    let t18810 = 0.3071625e0_f64 * t18742 - t11204 + 0.26574814814814814815e0_f64 * t14702 - t14868 - t14870 + 0.91285185185185185187e-1_f64 * t11211 - 0.76790625e-1_f64 * t18747 + 0.3071625e0_f64 * t18749 + 0.15358125e0_f64 * t18752 + 0.142419375e1_f64 * t18755 - 0.1898925e1_f64 * t18757;
    let t18832 = 0.11958666666666666667e1_f64 * t18227 + 0.36514074074074074073e-1_f64 * t14818 - 0.27385555555555555556e-1_f64 * t18515 + 0.36514074074074074075e-1_f64 * t18497 + 0.16431333333333333333e0_f64 * t18518 + 0.13287407407407407408e0_f64 * t11137 + 0.59793333333333333334e0_f64 * t18239 - 0.54771111111111111112e-1_f64 * t18503 - 0.16431333333333333333e0_f64 * t18500 + 0.32862666666666666666e0_f64 * t18510 + 0.49293999999999999999e0_f64 * t18508;
    (t18810, t18832)
}
