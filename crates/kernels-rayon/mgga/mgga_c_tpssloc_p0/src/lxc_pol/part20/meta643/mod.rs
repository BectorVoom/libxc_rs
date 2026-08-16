//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta643 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2355;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2356;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2357;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta643(t13823: f64, t2960: f64, t13816: f64, t2970: f64, t973: f64, t13828: f64, t10224: f64, t4522: f64, t13895: f64, t1599: f64, t2402: f64, t13908: f64, t10263: f64, t4528: f64, t12606: f64, t2989: f64, t10241: f64, t13861: f64, t1600: f64, t2986: f64, t2988: f64, t3008: f64, t3014: f64, t343: f64, t42554: f64, t43061: f64, t4514: f64, t4540: f64, t4543: f64, t4546: f64, t344: f64, t43052: f64, t4343: f64, t2978: f64, t4338: f64, t697: f64, t43053: f64, t13542: f64, t13779: f64, t13546: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48297, t48302, t48317, t48321, t48329, t48336, t48338) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2355(t13823, t2960, t13816, t2970, t973, t13828, t10224, t4522, t13895, t1599, t2402, t13908);
        let t48361 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2356(t48338, t10263, t4528, t12606, t2989, t10241, t13861, t1600, t2986, t2988, t3008, t3014, t343, t42554, t43061, t4514, t4540, t4543, t4546, t48329, t48336, t973);
        let (t48374, t48379, t48382, t48384, t48387) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2357(t2986, t344, t43052, t4343, t2978, t4338, t697, t43053, t4514, t13542, t13779, t13546);
    (t48297, t48302, t48317, t48321, t48361, t48374, t48379, t48382, t48384, t48387)
}
