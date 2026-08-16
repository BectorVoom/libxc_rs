//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta95 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk606;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk607;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk608;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk609;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk610;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk611;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk612;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta95<F: Float>(t2006: F, t539: F, t1998: F, t562: F, t214: F, t1985: F, t553: F, t544: F, t1378: F, t1375: F, t1989: F, t568: F, t533: F, t1390: F, t1983: F, t1401: F, t1873: F, t50: F, t56: F, t63: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2007, t2009, t2010, t2011, t2013) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk606::<F>(t2006, t539, t1998, t562, t214, t1985, t553);
        let t2015 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk607::<F>(t2011, t2013, t544);
        let t2016 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk608::<F>(t1378, t2015);
        let t2018 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk609::<F>(t1375, t1989, t2007, t2016, t568);
        let t2019 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk610::<F>(t2018, t533);
        let t2020 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk611::<F>(t1390, t2019);
        let (t2021, t2028, t2108) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk612::<F>(t1983, t2020, t1401, t1873, t50, t56, t63);
    (t2007, t2009, t2010, t2013, t2015, t2016, t2018, t2019, t2020, t2021, t2028, t2108)
}
