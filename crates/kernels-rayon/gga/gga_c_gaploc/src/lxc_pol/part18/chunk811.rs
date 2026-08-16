//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 811/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk811(t1457: f64, t7245: f64, t1445: f64, t7259: f64, t2089: f64, t2530: f64, t723: f64, t1645: f64, t1885: f64, t7499: f64, t7227: f64, t1880: f64, t733: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7751 = t1457 * t7245;
    let t7756 = t1445 * t7259;
    let t7759 = t1445 * t7245;
    let t7764 = t2089 * t2530;
    let t7765 = t7764 * t723;
    let t7766 = t1445 * t7765;
    let t7769 = t1645 * t1885;
    let t7772 = t1457 * t7499;
    let t7775 = t1457 * t7227;
    let t7778 = t733 * t1880;
    (t7751, t7756, t7759, t7766, t7769, t7772, t7775, t7778)
}
