//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta664 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2088;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2089;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta664(t225: f64, t3787: f64, t562: f64, t22751: f64, t26385: f64, t26389: f64, t26467: f64, t6914: f64, t26426: f64, t81046: f64, t22690: f64, t7732: f64, t81195: f64, t22832: f64, t5234: f64, t1336: f64, t22759: f64, t5252: f64, t836: f64, t5293: f64, t80820: f64, t1831: f64, t80869: f64, t22783: f64, t5314: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t91005, t91006, t91011, t91065, t91077, t91078, t91081) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2088(t225, t3787, t562, t22751, t26385, t26389, t26467, t6914, t26426, t81046, t22690, t7732, t81195);
        let (t91100, t91114, t91121, t91136, t91137) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2089(t22832, t5234, t1336, t22759, t5252, t836, t5293, t80820, t1831, t80869, t22783, t5314);
    (t91005, t91006, t91011, t91065, t91077, t91078, t91081, t91100, t91114, t91121, t91136, t91137)
}
