//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1012/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1012(t10820: f64, t10931: f64, t10930: f64, t9982: f64, t2676: f64, t8775: f64, t2536: f64, t3038: f64, t787: f64, t2028: f64, t2679: f64, t3005: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10932 = t10931 * t10820;
    let t10934 = 0.27606906686822939767e2_f64 * t10930 * t10932;
    let t10935 = 0.63904876589867916128e-1_f64 * t9982;
    let t10937 = 0.11916829983950142223e0_f64 * t8775 * t2676;
    let t10938 = t2536 * t3038;
    let t10939 = t787 * t10938;
    let t10941 = 0.39722766613167140743e-1_f64 * t10939 * t2028;
    let t10942 = t3005 * t2679;
    (t10932, t10934, t10935, t10937, t10938, t10939, t10941, t10942)
}
