//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 842/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk842<F: Float>(t40641: F, t43072: F, t44855: F, t44857: F, t44860: F, t44861: F, t44862: F, t44863: F, t44864: F, t44865: F, t739: F, t1022: F, t39048: F, t787: F, t14373: F, t14374: F, t2639: F, t313: F, t314: F, t317: F, t45606: F, t45608: F, t45611: F, t45614: F, t45617: F, t45627: F, t45630: F, t45633: F, t45636: F, t45639: F, t47377: F, t47378: F, t47379: F, t50130: F, t531: F, t568: F, t7427: F, t7573: F, t769: F, t784: F, t797: F, t833: F, t836: F) -> (F, F, F) {
    let t50182 = t44855 - t44857 + 2.0 * t43072 - 2.0 * t40641 + t44860 + t44861 - t44862 + t44863 - t44864 - t44865;
    let t50183 = t739 * t50182;
    let t50194 = t787 * t39048 * t1022;
    let t50208 = t45606 - t45608 - t45611 - 0.35750489951850426669e0 * t797 * t531 * t50183 + 0.23005755572352449806e1 * t833 * t568 * t836 * t50182 + 0.23833659967900284446e0 * t14374 * t784 + t45614 - t45617 - 0.50050685932590597338e1 * t50194 * t2639 + t47377 - t47378 - t45627 + t45630 - t45633 - 0.12423108009070322895e3 * t7427 * t7573 * t50130 - t45636 + t45639 + 0.35750489951850426669e0 * t769 * t14373 * t317 + 0.35750489951850426669e0 * t313 * t314 * t50182 * t317 + 0.59584149919750711116e-1 * t47379;
    (t50182, t50183, t50208)
}
