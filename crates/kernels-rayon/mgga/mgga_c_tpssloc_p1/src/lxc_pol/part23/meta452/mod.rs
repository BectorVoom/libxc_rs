//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta452 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1302;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1303;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1304;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta452(t40: f64, t52: f64, t16549: f64, t20217: f64, t2433: f64, t40632: f64, t4080: f64, t5398: f64, t73: f64, t75836: f64, t75847: f64, t75912: f64, t16563: f64, t2440: f64, t40647: f64, t4087: f64, t76: f64, zeta_threshold: f64, t157: f64, t182: f64, t58057: f64, t1530: f64, t193: f64, t20756: f64, t39529: f64, t40779: f64, t40784: f64, t40790: f64, t40793: f64, t40797: f64, t75894: f64, t75895: f64, t75900: f64, t75901: f64, t870: f64, t20816: f64, t4205: f64, t67230: f64, t67243: f64, t58972: f64, t67463: f64, t17116: f64, t1877: f64, t2522: f64, t28248: f64, t39549: f64, t39563: f64, t39585: f64, t39590: f64, t40799: f64, t40801: f64, t40803: f64, t5664: f64, t59564: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t75916, t75928) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1302(t40, t52, t16549, t20217, t2433, t40632, t4080, t5398, t73, t75836, t75847, t75912, t16563, t2440, t40647, t4087, t76, zeta_threshold);
        let (t75929, t75932, t75933, t75934) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1303(t75916, t75928, t157, t182, t58057, t1530, t193, t20756, t39529, t40779, t40784, t40790, t40793, t40797, t75894, t75895, t75900, t75901, t870);
        let (t75939, t75940, t75941, t75942, t75943, t75947) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1304(t20816, t4205, t67230, t67243, t58972, t67463, t17116, t1877, t2522, t28248, t39549, t39563, t39585, t39590, t40799, t40801, t40803, t5664, t59564);
    (t75929, t75932, t75933, t75934, t75939, t75940, t75941, t75942, t75943, t75947)
}
