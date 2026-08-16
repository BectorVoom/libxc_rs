//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 869/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk869(t1197: f64, t7590: f64, t1208: f64, t52: f64, t7457: f64, t290: f64, t5009: f64, t35462: f64, t1201: f64, t1209: f64, t19039: f64, t28558: f64, t28603: f64, t28652: f64, t28660: f64, t28677: f64, t28680: f64, t292: f64, t31462: f64, t31535: f64, t33903: f64, t33925: f64, t33947: f64, t33948: f64, t35358: f64, t35402: f64, t35406: f64, t35467: f64, t35872: f64, t35879: f64, t35887: f64, t4113: f64, t5265: f64, t7003: f64, t7607: f64) -> (f64, f64, f64, f64) {
    let t35890 = t7590 * t1197;
    let t35902 = t52 * t7457 * t1208;
    let t35908 = t290 * t5009;
    let t35909 = t35908 * t35462;
    let t35914 = -0.21340587982578139066e0_f64 * t7607 * t35467 + 0.18125821328051150223e0_f64 * t28680 * t35872 + 0.26675734978222673832e-1_f64 * t33948 * t35406 + 0.18125821328051150223e0_f64 * t28652 * t35879 - 0.18125821328051150223e0_f64 * t28660 * t35872 - 0.18125821328051150223e0_f64 * t28677 * t35879 + 0.22914129771549286116e-1_f64 * t31535 * t35887 + 0.20527106943485609994e0_f64 * t19039 * t35890 - 0.27369475924647479993e1_f64 * t292 * t35402 + 0.54738951849294959985e1_f64 * t1201 * t35402 + 0.30209702213418583705e-1_f64 * t28558 * t35358 - 0.30209702213418583705e-1_f64 * t28603 * t35358 - 0.22653425206514361674e0_f64 * t31462 * t35902 - 0.10263553471742804997e0_f64 * t5265 * t7590 * t1209 - t33903 + t33925 + t33947 - 0.45828259543098572232e-1_f64 * t7003 * t35909 + 0.15276086514366190744e-1_f64 * t4113 * t35909;
    (t35890, t35902, t35908, t35914)
}
