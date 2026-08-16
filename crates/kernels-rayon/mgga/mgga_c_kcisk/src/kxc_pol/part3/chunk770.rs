//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 770/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk770(t4265: f64, t5251: f64, t10487: f64, t702: f64, t10441: f64, t5248: f64, t1919: f64, t3293: f64, t5254: f64, t10449: f64, t1920: f64, t5261: f64) -> (f64, f64, f64, f64, f64) {
    let t11830 = t4265 * t5251;
    let t11832 = t702 * t10487;
    let t11834 = t5248 * t11832 * t10441;
    let t11838 = t1919 * t5254 * t3293;
    let t11842 = t1919 * t1920 * t10449;
    let t11851 = t4265 * t5261;
    (t11830, t11834, t11838, t11842, t11851)
}
