//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1324/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1324<F: Float>(t14630: F, t4772: F, t56775: F, t935: F, t1: F, t1235: F, t14525: F, t14640: F, t17047: F, t24574: F, t24989: F, t25561: F, t25564: F, t2668: F, t2672: F, t2721: F, t2722: F, t2812: F, t2813: F, t297: F, t313: F, t32576: F, t3623: F, t3907: F, t3917: F, t42152: F, t42157: F, t51400: F, t51452: F, t51461: F, t51502: F, t56708: F, t56862: F, t56881: F, t57537: F, t57545: F, t8002: F, t914: F, t930: F, t953: F) -> F {
    let t57718 = t14630 * t4772;
    let t57738 = t56775 * t935;
    let t57756 = -F::new(0.30228422675018518374e-1) * t953 * t56862 - F::new(0.2686970904446090522e0) * t953 * t57545 - F::new(0.45352564237957702055e6) * t51452 + F::new(0.45352564237957702055e6) * t51461 + F::new(0.30228422675018518373e0) * t953 * t57537 - F::new(0.4678438591588217436e2) * t2812 * t2813 * t57718 - F::new(0.35163949364965747848e4) * t3917 * t14640 * t56881 - F::new(0.18583473745796456084e3) * t3907 * t14525 * t8002 * t4772 - F::new(0.45440405106024376544e1) * t2721 * t2722 * t57718 - F::new(0.30972456242994093473e2) * t2668 * t3623 * t17047 + F::new(0.13909058383662012568e1) * t930 * t914 * t56708 + F::new(0.81145531355560548285e7) * t24574 * t313 * t57738 * t2672 - F::new(0.38640729216933594422e6) * t24989 * t313 * t57738 * t297 - F::new(0.67174272611152263053e-2) * t42152 + F::new(0.1343485452223045261e-1) * t42157 + F::new(0.59710464543246456046e-2) * t32576 - F::new(0.61944912485988186948e2) * t51502 + F::new(0.45352564237957702055e6) * t25561 * t51400 * t25564 * t1235 * t1;
    t57756
}
