//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1285/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1285(t2684: f64, t32897: f64, t7585: f64, t326: f64, t32803: f64, t825: f64, t10867: f64, t2634: f64, t4673: f64, t7427: f64, t7573: f64, t10930: f64, t10931: f64) -> (f64, f64, f64, f64, f64) {
    let t33832 = 0.87421871174939309262e2_f64 * t2684 * t7585 * t32897;
    let t33835 = 0.18404604457881959845e2_f64 * t825 * t326 * t32803;
    let t33838 = 0.33367123955060398226e1_f64 * t10867 * t4673 * t2634;
    let t33841 = 0.12423108009070322895e3_f64 * t7427 * t7573 * t32897;
    let t33844 = 0.55213813373645879534e2_f64 * t10930 * t10931 * t32897;
    (t33832, t33835, t33838, t33841, t33844)
}
