//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 885/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk885(t7487: f64, t9726: f64, t2019: f64, t2020: f64, t9754: f64, t2010: f64, t2012: f64, t5960: f64, t1704: f64, t236: f64, t495: f64, t7230: f64, t9188: f64) -> (f64, f64, f64, f64) {
    let t44854 = t7487 * t9726;
    let t44857 = t2019 * t2020 * t9754;
    let t44860 = t2010 * t2012 * t5960;
    let t44866 = t7230 * t9188 * t236 * t1704 * t495;
    (t44854, t44857, t44860, t44866)
}
