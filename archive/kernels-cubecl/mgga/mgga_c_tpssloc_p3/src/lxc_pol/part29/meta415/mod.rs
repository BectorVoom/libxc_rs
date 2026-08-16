//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta415 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1683;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1684;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1685;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1686;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1687;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1688;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta415<F: Float>(t16153: F, t3870: F, t820: F, t3799: F, t5289: F, t11984: F, t15876: F, t15878: F, t15880: F, t15887: F, t15888: F, t15889: F, t15891: F, t15894: F, t15896: F, t15898: F, t15910: F, t9457: F, t9476: F, t9484: F, t9780: F, t12044: F, t12048: F, t12057: F, t12059: F, t12087: F, t12094: F, t15911: F, t15915: F, t15916: F, t15917: F, t15923: F, t15927: F, t15928: F, t9789: F, t9793: F, t9797: F, t12103: F, t12105: F, t12109: F, t12114: F, t12116: F, t12118: F, t12123: F, t15970: F, t15972: F, t15973: F, t15974: F, t15975: F, t15976: F, t15978: F, t9820: F, t9824: F, t2371: F, t5154: F, t12134: F, t12136: F, t12138: F, t5151: F, t67: F, t758: F, t12142: F, t12127: F, t12133: F, t12141: F, t15980: F, t15983: F, t15985: F, t15987: F, t15988: F, t9853: F, t9859: F, t225: F, t1345: F, t68: F, t1799: F, t1995: F, t3734: F, t1365: F, t5187: F, t1307: F, t3719: F, t5279: F, t1347: F, t16018: F, t1348: F, t1819: F, t1821: F, t3839: F, t3844: F, t3847: F, t5272: F, t5278: F, t5280: F, t5283: F, t546: F, t548: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t16155, t16159, t16160) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1683::<F>(t16153, t3870, t820, t3799, t5289, t11984, t15876, t15878, t15880, t15887, t15888, t15889, t15891, t15894, t15896, t15898, t15910, t9457, t9476, t9484, t9780);
        let t16161 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1684::<F>(t12044, t12048, t12057, t12059, t12087, t12094, t15911, t15915, t15916, t15917, t15923, t15927, t15928, t9789, t9793, t9797);
        let t16163 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1685::<F>(t12103, t12105, t12109, t12114, t12116, t12118, t12123, t15970, t15972, t15973, t15974, t15975, t15976, t15978, t9820, t9824);
        let (t16165, t16166, t16167, t16168, t16171, t16172, t16173) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1686::<F>(t2371, t5154, t12134, t12136, t12138, t5151, t67, t758, t12142, t12127, t12133, t12141, t15980, t15983, t15985, t15987, t15988, t9853, t9859);
        let (t16176, t16186, t16192, t16195) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1687::<F>(t16160, t16161, t16163, t16173, t225, t1345, t68, t1799, t1995, t3734, t1365, t5187);
        let t16205 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1688::<F>(t1307, t16195, t3719, t5279, t1347, t16018, t1345, t1348, t16176, t16186, t16192, t1819, t1821, t3839, t3844, t3847, t5272, t5278, t5280, t5283, t546, t548);
    (t16155, t16159, t16165, t16166, t16167, t16168, t16171, t16172, t16205)
}
