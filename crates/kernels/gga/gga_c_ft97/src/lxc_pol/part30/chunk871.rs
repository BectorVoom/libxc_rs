//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 871/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk871<F: Float>(t1196: F, t683: F, t33436: F, t1208: F, t2035: F, t7590: F, t1200: F, t2725: F, t285: F, t52: F, t7457: F, t1201: F, t19101: F, t19107: F, t19132: F, t19135: F, t291: F, t292: F, t31462: F, t31465: F, t31539: F, t33906: F, t33928: F, t33934: F, t33941: F, t35368: F, t35386: F, t35438: F, t35505: F, t35902: F, t35917: F, t35924: F, t5265: F, t7006: F, t7009: F) -> (F, F, F, F, F, F, F, F) {
    let t35928 = t683 * t1196;
    let t35929 = t33436 * t35928;
    let t35932 = t683 * t1208;
    let t35941 = t2035 * t7590 * t1196;
    let t35945 = t2035 * t7590 * t1208;
    let t35952 = t1200 * t2725;
    let t35955 = t285 * t2725;
    let t35961 = t52 * t7457 * t1196;
    let t35970 = F::new(0.10594292039522084817e-1) * t35917 * t31539 + F::new(0.12081826776807659559e1) * t292 * t35505 - F::new(0.12081826776807659559e1) * t1201 * t35505 - F::new(0.13684737962323739996e1) * t5265 * t35924 * t291 + F::new(0.80027204934668021496e-1) * t33934 * t35929 - F::new(0.12004080740200203224e0) * t33941 * t33436 * t35932 + F::new(0.72503285312204600893e0) * t7006 * t35368 - F::new(0.72503285312204600893e0) * t7009 * t35368 + F::new(0.41054213886971219988e0) * t19132 * t35941 - F::new(0.20527106943485609994e0) * t19135 * t35945 - F::new(0.82108427773942439976e0) * t19101 * t35941 + F::new(0.41054213886971219988e0) * t19107 * t35945 - F::new(0.29389470585448002138e-1) * t35952 * t35438 + F::new(0.29389470585448002138e-1) * t35955 * t35438 + F::new(0.21188584079044169633e-1) * t31465 * t35386 - F::new(0.45306850413028723348e0) * t33928 * t35961 + F::new(0.22653425206514361674e0) * t31465 * t35902 - F::new(0.42377168158088339266e-1) * t31462 * t35386 + F::new(0.45306850413028723348e0) * t33906 * t35961;
    (t35928, t35929, t35932, t35941, t35952, t35955, t35961, t35970)
}
