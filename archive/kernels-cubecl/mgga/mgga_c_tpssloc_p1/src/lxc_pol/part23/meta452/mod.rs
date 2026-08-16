//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta452 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1302;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1303;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1304;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta452<F: Float>(t40: F, t52: F, t16549: F, t20217: F, t2433: F, t40632: F, t4080: F, t5398: F, t73: F, t75836: F, t75847: F, t75912: F, t16563: F, t2440: F, t40647: F, t4087: F, t76: F, zeta_threshold: F, t157: F, t182: F, t58057: F, t1530: F, t193: F, t20756: F, t39529: F, t40779: F, t40784: F, t40790: F, t40793: F, t40797: F, t75894: F, t75895: F, t75900: F, t75901: F, t870: F, t20816: F, t4205: F, t67230: F, t67243: F, t58972: F, t67463: F, t17116: F, t1877: F, t2522: F, t28248: F, t39549: F, t39563: F, t39585: F, t39590: F, t40799: F, t40801: F, t40803: F, t5664: F, t59564: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t75916, t75928) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1302::<F>(t40, t52, t16549, t20217, t2433, t40632, t4080, t5398, t73, t75836, t75847, t75912, t16563, t2440, t40647, t4087, t76, zeta_threshold);
        let (t75929, t75932, t75933, t75934) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1303::<F>(t75916, t75928, t157, t182, t58057, t1530, t193, t20756, t39529, t40779, t40784, t40790, t40793, t40797, t75894, t75895, t75900, t75901, t870);
        let (t75939, t75940, t75941, t75942, t75943, t75947) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1304::<F>(t20816, t4205, t67230, t67243, t58972, t67463, t17116, t1877, t2522, t28248, t39549, t39563, t39585, t39590, t40799, t40801, t40803, t5664, t59564);
    (t75929, t75932, t75933, t75934, t75939, t75940, t75941, t75942, t75943, t75947)
}
