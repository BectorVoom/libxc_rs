//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1443/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1443(t28936: f64, t28940: f64, t28944: f64, t28946: f64, t33820: f64, t33824: f64, t33826: f64, t33829: f64, t33832: f64, t33835: f64, t33838: f64, t33841: f64, t33844: f64, t33846: f64, t33848: f64, t33851: f64) -> f64 {
    let t39299 = -t33820 + t33824 - 0.76685851907841499354e0_f64 * t28936 - 0.38342925953920749677e0_f64 * t28940 + t28944 + t28946 + t33826 - t33829 + t33832 - t33835 - t33838 - t33841 + t33844 + t33846 - t33848 + t33851;
    t39299
}
