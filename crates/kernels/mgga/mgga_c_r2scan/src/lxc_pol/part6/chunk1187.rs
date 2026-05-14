//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1187/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1187<F: Float>(t183: F, t189: F, t18956: F, t21529: F, t21531: F, t21533: F, t21535: F, t21540: F, t21542: F, t21544: F, t21546: F, t5549: F, t689: F, t1399: F, t1814: F, t1823: F, t1831: F, t1923: F, t1937: F, t1938: F, t2000: F, t201: F, t2029: F, t2030: F, t208: F, t2090: F, t219: F, t226: F, t390: F, t4741: F, t5305: F, t5632: F, t5636: F, t5785: F, t625: F, t686: F, t741: F, t750: F) -> (F, F, F) {
    let t21743 = 1.0 * t183 * (0.59077666666666666667e2 * t21529 - 0.101276e3 * t21531 + 0.22505777777777777778e2 * t21533 - 0.26256740740740740741e2 * t21535 - 0.68258333333333333335e0 * t21540 + 0.65528000000000000001e1 * t21542 - 0.2426962962962962963e1 * t21544 + 0.21235925925925925926e1 * t21546 + 0.8519950617283950617e0 * t18956) * t189;
    let t21747 = t689 * t5549;
    let t21767 = 0.6947415143435175149e4 * t5785 * t5632 * t1923 + 0.17757530864197530864e0 * t625 * t2090 * t201 * t208 + 0.5622597711267568807e-1 * t625 * t2090 * t219 * t226 + 0.13698666666666666666e0 * t625 * t2000 * t2030 - t21743 - 0.66090502947826842109e1 * t390 * t5636 * t5632 - 0.22030167649275614036e1 * t390 * t1937 * t21747 - 0.6609050294782684211e1 * t390 * t686 * t2029 * t1938 - 0.19263893255070628432e1 * t390 * t5305 + 0.13494234507042165137e0 * t4741 * t741 - 0.19977370783036207262e1 * t4741 * t750 + 0.12842595503380418954e1 * t1399 * t1814 + 0.76050639865105016044e2 * t1399 * t1823 - 0.86748650402413918737e-1 * t1399 * t1831;
    (t21743, t21747, t21767)
}
