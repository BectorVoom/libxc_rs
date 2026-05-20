//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1609/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1609<F: Float>(t12948: F, t13058: F, t12937: F, t3172: F, t3711: F, t13080: F, t5384: F, t1231: F, t12898: F, t3651: F, t3655: F, t43830: F, t43832: F, t43837: F, t43841: F, t43845: F, t43849: F, t43858: F, t43862: F, t43865: F, t43871: F, t43877: F) -> (F, F, F, F, F, F) {
    let t44283 = t13058 * t12948;
    let t44286 = t3711 * t3172 * t12937;
    let t44289 = t5384 * t3172 * t13080;
    let t44291 = t1231 * t12898;
    let t44293 = t3651 * t3655;
    let t44306 = -F::cast_from(0.12345679012345679012e-1_f64) * t43858 - F::cast_from(0.24691358024691358025e-1_f64) * t43862 - F::cast_from(0.66666666666666666668e-1_f64) * t43830 - F::cast_from(0.14814814814814814815e-1_f64) * t43865 + F::cast_from(0.22222222222222222222e-1_f64) * t43832 + F::cast_from(0.55555555555555555555e-1_f64) * t43837 - F::cast_from(0.16666666666666666666e-1_f64) * t43871 - F::cast_from(0.22222222222222222222e-1_f64) * t43841 + F::new(0.3e0) * t43845 + F::cast_from(0.50000000000000000001e-1_f64) * t43877 + F::cast_from(0.66666666666666666668e-1_f64) * t43849;
    (t44283, t44286, t44289, t44291, t44293, t44306)
}
