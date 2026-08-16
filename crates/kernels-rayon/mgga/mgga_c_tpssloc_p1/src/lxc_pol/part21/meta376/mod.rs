//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta376 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1826;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1827;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1828;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta376(t13779: f64, t4343: f64, t2986: f64, t134: f64, t2978: f64, t344: f64, t4338: f64, t10190: f64, t4514: f64, t13528: f64, t4510: f64, t13532: f64, t10213: f64, t60: f64, t13537: f64, t10186: f64, t10192: f64, t10226: f64, t10229: f64, t13770: f64, t4511: f64, t4515: f64, t4519: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13780, t13782, t13783, t13784) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1826(t13779, t4343, t2986, t134, t2978, t344);
        let (t13785, t13787, t13788, t13790, t13791, t13794, t13797, t13798) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1827(t13784, t4338, t2986, t10190, t4514, t13528, t4510, t13532, t10213, t60, t344);
        let t13804 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1828(t13537, t13798, t10186, t10192, t10226, t10229, t13770, t13782, t13787, t13790, t13791, t13794, t2986, t4511, t4515, t4519);
    (t13780, t13782, t13783, t13784, t13785, t13787, t13788, t13790, t13797, t13798, t13804)
}
