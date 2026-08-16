//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1307/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1307(t1203: f64, t1214: f64, t12690: f64, t12732: f64, t1287: f64, t2144: f64, t21483: f64, t26884: f64, t26889: f64, t26918: f64, t26933: f64, t26944: f64, t26963: f64, t26969: f64, t26970: f64, t26979: f64, t26999: f64, t27005: f64, t27029: f64, t29159: f64, t29204: f64, t3552: f64, t3576: f64, t7629: f64, t7636: f64, t7637: f64, t7643: f64, t7648: f64, t7652: f64, t7654: f64, t7659: f64, t7660: f64, t96966: f64, t96979: f64, t96981: f64, t96982: f64, t96986: f64, t97011: f64) -> f64 {
    let t97015 = 0.52041769129231196772e1_f64 * t96966 * t7654 - 0.26020884564615598386e1_f64 * t26918 * t26933 + 0.65854491829355115987e0_f64 * t12690 * t2144 - 0.4336814094102599731e0_f64 * t7659 * t7660 * t12732 * t1287 + 0.19756347548806534796e1_f64 * t3552 * t7629 - 0.52041769129231196772e1_f64 * t96979 * t96981 * t96982 + 0.52041769129231196772e1_f64 * t96986 * t96981 * t21483 + 0.26020884564615598386e1_f64 * t7643 * t7637 * t26884 * t1214 + 0.52041769129231196772e1_f64 * t26979 * t27029 + 0.10408353825846239354e2_f64 * t7636 * t7652 * t26944 * t1203 - 0.26020884564615598386e1_f64 * t29204 * t26963 + 0.39512695097613069591e1_f64 * t26999 * t3576 + 0.15612530738769359031e2_f64 * t7643 * t26969 * t26970 * t1214 - 0.13010442282307799193e1_f64 * t7648 * t27005 - 0.26020884564615598386e1_f64 * t26889 * t97011 * t29159;
    t97015
}
