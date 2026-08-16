//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1609/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1609(t12948: f64, t13058: f64, t12937: f64, t3172: f64, t3711: f64, t13080: f64, t5384: f64, t1231: f64, t12898: f64, t3651: f64, t3655: f64, t43830: f64, t43832: f64, t43837: f64, t43841: f64, t43845: f64, t43849: f64, t43858: f64, t43862: f64, t43865: f64, t43871: f64, t43877: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44283 = t13058 * t12948;
    let t44286 = t3711 * t3172 * t12937;
    let t44289 = t5384 * t3172 * t13080;
    let t44291 = t1231 * t12898;
    let t44293 = t3651 * t3655;
    let t44306 = -0.12345679012345679012e-1_f64 * t43858 - 0.24691358024691358025e-1_f64 * t43862 - 0.66666666666666666668e-1_f64 * t43830 - 0.14814814814814814815e-1_f64 * t43865 + 0.22222222222222222222e-1_f64 * t43832 + 0.55555555555555555555e-1_f64 * t43837 - 0.16666666666666666666e-1_f64 * t43871 - 0.22222222222222222222e-1_f64 * t43841 + 0.3e0_f64 * t43845 + 0.50000000000000000001e-1_f64 * t43877 + 0.66666666666666666668e-1_f64 * t43849;
    (t44283, t44286, t44289, t44291, t44293, t44306)
}
