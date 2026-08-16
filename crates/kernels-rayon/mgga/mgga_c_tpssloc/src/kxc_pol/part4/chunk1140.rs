//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1140/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1140(t11211: f64, t11372: f64, t14702: f64, t14705: f64, t14711: f64, t18742: f64, t18747: f64, t18749: f64, t18752: f64, t18755: f64, t18757: f64, t3270: f64, t5999: f64) -> (f64, f64) {
    let t18759 = 0.16504875e0_f64 * t18742 - t11372 + 0.26837777777777777779e0_f64 * t14702 - t14705 - t14711 + 0.91983333333333333333e-1_f64 * t11211 - 0.412621875e-1_f64 * t18747 + 0.16504875e0_f64 * t18749 + 0.82524375e-1_f64 * t18752 + 0.19419375e1_f64 * t18755 - 0.258925e1_f64 * t18757;
    let t18761 = t3270 * t5999;
    (t18759, t18761)
}
