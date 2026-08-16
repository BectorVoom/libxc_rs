//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta415 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1683;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1684;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1685;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1686;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1687;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1688;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta415(t16153: f64, t3870: f64, t820: f64, t3799: f64, t5289: f64, t11984: f64, t15876: f64, t15878: f64, t15880: f64, t15887: f64, t15888: f64, t15889: f64, t15891: f64, t15894: f64, t15896: f64, t15898: f64, t15910: f64, t9457: f64, t9476: f64, t9484: f64, t9780: f64, t12044: f64, t12048: f64, t12057: f64, t12059: f64, t12087: f64, t12094: f64, t15911: f64, t15915: f64, t15916: f64, t15917: f64, t15923: f64, t15927: f64, t15928: f64, t9789: f64, t9793: f64, t9797: f64, t12103: f64, t12105: f64, t12109: f64, t12114: f64, t12116: f64, t12118: f64, t12123: f64, t15970: f64, t15972: f64, t15973: f64, t15974: f64, t15975: f64, t15976: f64, t15978: f64, t9820: f64, t9824: f64, t2371: f64, t5154: f64, t12134: f64, t12136: f64, t12138: f64, t5151: f64, t67: f64, t758: f64, t12142: f64, t12127: f64, t12133: f64, t12141: f64, t15980: f64, t15983: f64, t15985: f64, t15987: f64, t15988: f64, t9853: f64, t9859: f64, t225: f64, t1345: f64, t68: f64, t1799: f64, t1995: f64, t3734: f64, t1365: f64, t5187: f64, t1307: f64, t3719: f64, t5279: f64, t1347: f64, t16018: f64, t1348: f64, t1819: f64, t1821: f64, t3839: f64, t3844: f64, t3847: f64, t5272: f64, t5278: f64, t5280: f64, t5283: f64, t546: f64, t548: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16155, t16159, t16160) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1683(t16153, t3870, t820, t3799, t5289, t11984, t15876, t15878, t15880, t15887, t15888, t15889, t15891, t15894, t15896, t15898, t15910, t9457, t9476, t9484, t9780);
        let t16161 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1684(t12044, t12048, t12057, t12059, t12087, t12094, t15911, t15915, t15916, t15917, t15923, t15927, t15928, t9789, t9793, t9797);
        let t16163 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1685(t12103, t12105, t12109, t12114, t12116, t12118, t12123, t15970, t15972, t15973, t15974, t15975, t15976, t15978, t9820, t9824);
        let (t16165, t16166, t16167, t16168, t16171, t16172, t16173) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1686(t2371, t5154, t12134, t12136, t12138, t5151, t67, t758, t12142, t12127, t12133, t12141, t15980, t15983, t15985, t15987, t15988, t9853, t9859);
        let (t16176, t16186, t16192, t16195) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1687(t16160, t16161, t16163, t16173, t225, t1345, t68, t1799, t1995, t3734, t1365, t5187);
        let t16205 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1688(t1307, t16195, t3719, t5279, t1347, t16018, t1345, t1348, t16176, t16186, t16192, t1819, t1821, t3839, t3844, t3847, t5272, t5278, t5280, t5283, t546, t548);
    (t16155, t16159, t16165, t16166, t16167, t16168, t16171, t16172, t16205)
}
