//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta97 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk632;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk633;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk634;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk635;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk636;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk637;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk638;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk639;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta97(t2002: f64, t544: f64, t559: f64, t1992: f64, t2000: f64, t539: f64, t1998: f64, t562: f64, t214: f64, t1985: f64, t553: f64, t1378: f64, t1375: f64, t1989: f64, t568: f64, t533: f64, t1390: f64, t1983: f64, t113: f64, t1869: f64, t1876: f64, t1976: f64, t1980: f64, t510: f64, t574: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2003, t2006) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk632(t2002, t544, t559, t1992, t2000);
        let (t2007, t2009, t2010, t2011, t2013) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk633(t2006, t539, t1998, t562, t214, t1985, t553);
        let t2015 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk634(t2011, t2013, t544);
        let t2016 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk635(t1378, t2015);
        let t2018 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk636(t1375, t1989, t2007, t2016, t568);
        let t2019 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk637(t2018, t533);
        let t2020 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk638(t1390, t2019);
        let t2022 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk639(t1983, t2020, t113, t1869, t1876, t1976, t1980, t510, t574);
    (t2003, t2006, t2007, t2009, t2010, t2013, t2015, t2016, t2018, t2019, t2020, t2022)
}
