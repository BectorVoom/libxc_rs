//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta613 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2010;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2011;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta613(t3082: f64, t6759: f64, t344: f64, t607: f64, t1009: f64, t6740: f64, t23509: f64, t25651: f64, t23563: f64, t25650: f64, t6750: f64, t23482: f64, t3: f64, t23471: f64, t10889: f64, t23535: f64, t3033: f64, t1016: f64, t3034: f64, t1930: f64, t23418: f64, t3180: f64, t10401: f64, t23417: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t82885, t82892, t82895, t82911, t82914, t82926) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2010(t3082, t6759, t344, t607, t1009, t6740, t23509, t25651, t23563, t25650, t6750, t23482, t3);
        let (t82943, t82956, t82986, t83008, t83015) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2011(t23471, t23482, t10889, t23535, t3033, t1016, t3034, t1930, t23418, t3180, t10401, t23417);
    (t82885, t82892, t82895, t82911, t82914, t82926, t82943, t82956, t82986, t83008, t83015)
}
