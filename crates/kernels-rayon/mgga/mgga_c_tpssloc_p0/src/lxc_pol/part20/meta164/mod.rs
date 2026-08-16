//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta164 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1035;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1036;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1037;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1038;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1039;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta164(t1315: f64, t1341: f64, t1354: f64, t1363: f64, t1369: f64, t3733: f64, t3762: f64, t3763: f64, t3766: f64, t3770: f64, t3774: f64, t3778: f64, t3781: f64, t3783: f64, t3790: f64, t3795: f64, t3800: f64, t3803: f64, t3809: f64, t3853: f64, t3858: f64, t3864: f64, t3867: f64, t3872: f64, t3876: f64, t559: f64, t539: f64, t1373: f64, t225: f64, t1376: f64, t566: f64, t68: f64, t1385: f64, t3787: f64, t562: f64, t3793: f64, t1338: f64, t1372: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3879 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1035(t1315, t1341, t1354, t1363, t1369, t3733, t3762, t3763, t3766, t3770, t3774, t3778, t3781, t3783, t3790, t3795, t3800, t3803, t3809, t3853, t3858, t3864, t3867, t3872, t3876, t559);
        let (t3880, t3882) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1036(t3879, t539, t1373, t225);
        let t3887 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1037(t1376, t566, t68);
        let (t3888, t3889) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1038(t1385, t3887);
        let (t3897, t3898, t3901) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1039(t3787, t562, t3793, t1338, t1372);
    (t3879, t3880, t3882, t3887, t3888, t3889, t3897, t3898, t3901)
}
