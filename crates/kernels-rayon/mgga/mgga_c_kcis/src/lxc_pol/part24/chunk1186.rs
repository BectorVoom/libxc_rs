//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1186/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1186(t283: f64, t5025: f64, t27779: f64, t93435: f64, t26685: f64, t14443: f64, t27825: f64, t7703: f64, t95606: f64, t14447: f64, t27949: f64, t1245: f64, t27774: f64, t2909: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t95664 = t5025 * t283;
    let t95684 = t93435 * t27779;
    let t95686 = 0.61836467013888888889e-4_f64 * t26685 * t95684;
    let t95696 = 0.30891203703703703704e-3_f64 * t7703 * t14443 * t27825;
    let t95698 = 0.20612155671296296296e-4_f64 * t26685 * t95606;
    let t95764 = t7703 * t14447 * t27949;
    let t95775 = t7703 * t1245 * t2909 * t27774;
    (t95664, t95684, t95686, t95696, t95698, t95764, t95775)
}
