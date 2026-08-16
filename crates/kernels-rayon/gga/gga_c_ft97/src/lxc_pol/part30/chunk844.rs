//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 844/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk844(t10079: f64, t35558: f64, t1091: f64, t33715: f64, t2599: f64, t33759: f64, t2606: f64, t24793: f64, t6917: f64, t24412: f64, t6930: f64, t242: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t35559 = t10079 * t35558;
    let t35562 = t33715 * t1091;
    let t35563 = t2599 * t35562;
    let t35566 = t33759 * t1091;
    let t35567 = t2606 * t35566;
    let t35570 = t24793 * t6917;
    let t35573 = t24412 * t6930;
    let t35574 = t242 * t35573;
    (t35559, t35562, t35563, t35566, t35567, t35570, t35573, t35574)
}
