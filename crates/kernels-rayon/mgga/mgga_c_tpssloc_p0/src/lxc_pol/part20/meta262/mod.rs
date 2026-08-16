//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta262 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1404;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1405;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1406;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1407;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta262(t10231: f64, t2981: f64, t973: f64, t4509: f64, t984: f64, t2770: f64, t343: f64, t2244: f64, t2987: f64, t3008: f64, t2990: f64, t2250: f64, t2989: f64, t2988: f64, t2775: f64, t607: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10232, t10233, t10235, t10236) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1404(t10231, t2981, t973, t4509, t984, t2770, t343);
        let t10237 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1405(t10236, t2244);
        let (t10238, t10241, t10242, t10245) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1406(t10235, t10237, t2987, t3008, t2990, t2250, t2989);
        let (t10246, t10250) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1407(t10245, t2988, t2775, t607, t2250);
    (t10232, t10233, t10235, t10236, t10237, t10238, t10241, t10242, t10245, t10246, t10250)
}
