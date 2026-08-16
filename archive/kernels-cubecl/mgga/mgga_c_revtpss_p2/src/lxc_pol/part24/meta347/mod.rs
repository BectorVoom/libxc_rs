//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta347 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1206;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1207;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1208;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1209;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1210;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta347<F: Float>(t1609: F, t19330: F, t2924: F, t1622: F, t6173: F, t11452: F, t23705: F, t23451: F, t3014: F, t11574: F, t15189: F, t18919: F, t18924: F, t18934: F, t23479: F, t23483: F, t23487: F, t23490: F, t23501: F, t23505: F, t11560: F, t324: F, t11409: F, t11450: F, t11466: F, t15350: F, t15406: F, t1634: F, t19156: F, t23665: F, t23723: F, t23755: F, t23758: F, t23761: F, t23764: F, t23769: F, t2943: F, t2968: F, t2987: F, t3012: F, t311: F, t4685: F, t6177: F, t6206: F, t6209: F, t946: F, t23720: F, t300: F, t23455: F, t23459: F, t23562: F, t23564: F, t23567: F, t23570: F, t23698: F, t23652: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t23770, t23772, t23773, t23776, t23785, t23798) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1206::<F>(t1609, t19330, t2924, t1622, t6173, t11452, t23705, t23451, t3014, t11574, t15189, t18919, t18924, t18934, t23479, t23483, t23487, t23490, t23501, t23505);
        let (t23811, t23812) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1207::<F>(t11560, t15189, t18919, t18924, t18934, t23479, t23483, t23487, t23490, t23501, t23505, t324);
        let t23814 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1208::<F>(t11409, t11450, t11466, t15350, t15406, t1634, t19156, t23665, t23723, t23755, t23758, t23761, t23764, t23769, t23772, t23773, t23776, t23785, t23798, t23812, t2943, t2968, t2987, t3012, t311, t4685, t6177, t6206, t6209, t946);
        let (t23816, t23818, t23819) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1209::<F>(t23720, t23814, t300, t23812, t23455, t23459, t23562, t23564, t23567, t23570, t23665, t23698, t23769, t23772);
        let t23820 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1210::<F>(t23652, t23819);
    (t23770, t23772, t23773, t23776, t23785, t23798, t23811, t23816, t23818, t23820)
}
