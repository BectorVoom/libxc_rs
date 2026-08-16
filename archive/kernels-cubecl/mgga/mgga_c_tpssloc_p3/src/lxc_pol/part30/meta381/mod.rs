//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta381 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1447;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1448;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1449;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1450;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta381<F: Float>(t16684: F, t16686: F, t16698: F, t16720: F, t225: F, t1504: F, t68: F, t1891: F, t5527: F, t776: F, t4119: F, t4226: F, t5544: F, t845: F, t16662: F, t824: F, t1506: F, t228: F, t230: F, t4219: F, t4225: F, t4227: F, t4230: F, t5601: F, t5605: F, t5608: F, t822: F, t825: F, t232: F, t860: F, t2732: F, t5612: F, t1509: F, t1519: F, t829: F, t4234: F, t4282: F, t5550: F, t9573: F, t213: F, t221: F, t4128: F, t12986: F, t13002: F, t13005: F, t13010: F, t4127: F, t9526: F, t9540: F, t9542: F, t9547: F, t9572: F) -> (F, F, F, F, F, F, F, F) {
        let (t16723, t16729, t16737, t16740) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1447::<F>(t16684, t16686, t16698, t16720, t225, t1504, t68, t1891, t5527, t776, t4119, t4226);
        let t16752 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1448::<F>(t5544, t845, t776, t16662, t824, t1504, t1506, t16723, t16729, t16737, t16740, t228, t230, t4219, t4225, t4227, t4230, t5601, t5605, t5608, t822, t825);
        let (t16753, t16754, t16756, t16758, t16759, t16762, t16769) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1449::<F>(t16752, t232, t860, t2732, t5612, t1509, t1519, t829, t4234, t4282, t5550, t9573);
        let t16781 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1450::<F>(t213, t5527, t221, t776, t4119, t4128, t12986, t13002, t13005, t13010, t16769, t4127, t9526, t9540, t9542, t9547, t9572);
    (t16752, t16753, t16754, t16756, t16758, t16759, t16762, t16781)
}
