//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1148/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1148<F: Float>(t26983: F, t7635: F, t1210: F, t29193: F, t2142: F, t3153: F, t3601: F, t1203: F, t5464: F, t26894: F, t3588: F, t73: F, t1214: F, t12690: F, t12732: F, t1287: F, t2144: F, t21483: F, t26884: F, t26889: F, t26918: F, t26933: F, t26944: F, t26963: F, t26969: F, t26970: F, t26979: F, t26999: F, t27005: F, t27029: F, t29159: F, t29204: F, t3552: F, t3576: F, t7629: F, t7636: F, t7637: F, t7643: F, t7648: F, t7652: F, t7654: F, t7659: F, t7660: F) -> (F, F, F, F) {
    let t96966 = t26983 * t7635;
    let t96979 = t1210 * t29193;
    let t96981 = t2142 * t3601 * t3153;
    let t96982 = t5464 * t1203;
    let t96986 = t26894 * t29193;
    let t97010 = t2142 * t3588;
    let t97011 = t97010 * t73;
    let t97015 = 0.52041769129231196772e1 * t96966 * t7654 - 0.26020884564615598386e1 * t26918 * t26933 + 0.65854491829355115987e0 * t12690 * t2144 - 0.4336814094102599731e0 * t7659 * t7660 * t12732 * t1287 + 0.19756347548806534796e1 * t3552 * t7629 - 0.52041769129231196772e1 * t96979 * t96981 * t96982 + 0.52041769129231196772e1 * t96986 * t96981 * t21483 + 0.26020884564615598386e1 * t7643 * t7637 * t26884 * t1214 + 0.52041769129231196772e1 * t26979 * t27029 + 0.10408353825846239354e2 * t7636 * t7652 * t26944 * t1203 - 0.26020884564615598386e1 * t29204 * t26963 + 0.39512695097613069591e1 * t26999 * t3576 + 0.15612530738769359031e2 * t7643 * t26969 * t26970 * t1214 - 0.13010442282307799193e1 * t7648 * t27005 - 0.26020884564615598386e1 * t26889 * t97011 * t29159;
    (t96981, t97010, t97011, t97015)
}
