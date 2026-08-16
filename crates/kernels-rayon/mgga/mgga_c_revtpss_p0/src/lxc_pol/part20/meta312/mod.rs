//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta312 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1215;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1216;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1217;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1218;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta312(t12224: f64, t12237: f64, t12366: f64, t12381: f64, t12395: f64, t12413: f64, t12417: f64, t12561: f64, t12566: f64, t12579: f64, t12583: f64, t12594: f64, t12730: f64, t1287: f64, t487: f64, t12646: f64, t1280: f64, t1269: f64, t3588: f64, t1204: f64, t3781: f64, t1214: f64, t1209: f64, t5462: f64, t3601: f64, t3769: f64, t5477: f64, t3783: f64, t12690: f64, t12699: f64, t12702: f64, t12706: f64, t12709: f64, t12714: f64, t12717: f64, t12719: f64, t12723: f64, t12727: f64, t1285: f64, t1288: f64, t1291: f64, t3552: f64, t3670: f64, t3746: f64, t3755: f64, t3756: f64, t3770: f64, t3774: f64, t3784: f64, t490: f64, t5463: f64, t5478: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t12731 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1215(t12224, t12237, t12366, t12381, t12395, t12413, t12417, t12561, t12566, t12579, t12583, t12594);
        let t12732 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1216(t12730, t12731);
        let (t12734, t12737, t12741, t12744, t12747, t12748, t12751) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1217(t12732, t1287, t487, t12646, t1280, t1269, t3588, t1204, t3781, t1214, t1209, t5462);
        let (t12753, t12756, t12757, t12766) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1218(t1214, t3601, t3769, t1209, t5477, t3783, t12690, t12699, t12702, t12706, t12709, t12714, t12717, t12719, t12723, t12727, t12734, t12737, t12741, t12744, t12748, t12751, t1285, t1288, t1291, t3552, t3670, t3746, t3755, t3756, t3770, t3774, t3784, t490, t5463, t5478);
    (t12732, t12734, t12737, t12741, t12744, t12747, t12748, t12751, t12753, t12756, t12757, t12766)
}
