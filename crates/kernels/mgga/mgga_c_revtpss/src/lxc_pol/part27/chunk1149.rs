//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1149/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1149<F: Float>(t3566: F, t7627: F, t7642: F, t96873: F, t26948: F, t487: F, t8945: F, t26936: F, t3736: F, t7635: F, t1203: F, t1294: F, t1214: F, t1248: F, t12651: F, t1269: F, t12696: F, t1287: F, t2151: F, t2152: F, t26889: F, t26895: F, t26940: F, t26944: F, t26949: F, t26950: F, t26951: F, t26958: F, t26962: F, t26983: F, t26987: F, t26994: F, t3569: F, t5458: F, t7602: F, t7632: F, t7636: F, t7637: F, t7643: F, t7645: F, t7652: F, t97011: F) -> (F,) {
    let t97019 = t3566 * t7627;
    let t97034 = t7642 * t96873;
    let t97040 = t26948 * t487;
    let t97041 = t97040 * t8945;
    let t97050 = t26948 * t26936;
    let t97065 = t7635 * t3736;
    let t97066 = t3566 * t97065;
    let t97067 = t1203 * t1294;
    let t97072 = 0.26020884564615598386e1 * t26895 * t97011 * t5458 + 0.39512695097613069591e1 * t97019 * t3569 + 0.52041769129231196772e1 * t7636 * t7652 * t26962 * t1294 + 0.52041769129231196772e1 * t26994 * t7637 * t26958 * t1203 + 0.19756347548806534796e1 * t7602 * t12651 + 0.39512695097613069591e1 * t7632 * t12696 + 0.26020884564615598386e1 * t97034 * t7645 - 0.26020884564615598386e1 * t26983 * t1269 * t2152 - 0.78062653693846795158e1 * t97041 * t26950 * t1248 * t1287 - 0.52041769129231196772e1 * t26889 * t26940 * t1248 * t1287 - 0.78062653693846795158e1 * t97050 * t26951 + 0.15612530738769359031e2 * t26949 * t7652 * t26950 * t1294 - 0.10408353825846239354e2 * t7643 * t7652 * t26944 * t1214 + 0.52041769129231196772e1 * t7636 * t7652 * t26987 * t1203 - 0.20816707651692478709e2 * t97066 * t2151 * t97067 * t1214;
    (t97072,)
}
