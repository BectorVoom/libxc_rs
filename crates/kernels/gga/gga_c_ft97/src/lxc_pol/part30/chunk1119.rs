//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1119/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1119<F: Float>(t1200: F, t153047: F, t4088: F, t52: F, t7457: F, t173: F, t35870: F, t7470: F, t28680: F, t142696: F, t4113: F, t112071: F, t112268: F, t1208: F, t127456: F, t142704: F, t142737: F, t142739: F, t142744: F, t142746: F, t150321: F, t150331: F, t150758: F, t152981: F, t153025: F, t153035: F, t153039: F, t153044: F, t2344: F, t28558: F, t28603: F, t28652: F, t33426: F, t33928: F, t684: F, t70550: F) -> (F, F, F, F) {
    let t153048 = t1200 * t153047;
    let t153054 = t52 * t7457 * t4088;
    let t153060 = t7470 * t173 * t35870;
    let t153061 = t28680 * t153060;
    let t153063 = t4113 * t142696;
    let t153066 = -F::cast_from(0.54377463984153450669e0_f64) * t127456 * t152981 - F::cast_from(0.24167761770734866964e0_f64) * t28558 * t150321 + F::cast_from(0.36251642656102300446e0_f64) * t112071 * t153025 - F::cast_from(0.80027204934668021493e-1_f64) * t142704 * t33426 * t2344 * t1208 * t684 + F::cast_from(0.24167761770734866964e0_f64) * t28603 * t150321 - F::cast_from(0.14125722719362779755e-1_f64) * t153035 + F::cast_from(0.6041940442683716741e-1_f64) * t142737 - F::cast_from(0.6041940442683716741e-1_f64) * t142739 - F::cast_from(0.14500657062440920179e1_f64) * t28652 * t153039 - F::cast_from(0.41054213886971219988e0_f64) * t70550 * t153044 - F::cast_from(0.14125722719362779755e-1_f64) * t153048 * t150758 - F::cast_from(0.6041940442683716741e-1_f64) * t142744 + F::cast_from(0.6041940442683716741e-1_f64) * t142746 - F::cast_from(0.45306850413028723348e0_f64) * t33928 * t153054 - F::cast_from(0.36251642656102300446e0_f64) * t112268 * t153025 + F::cast_from(0.6041940442683716741e-1_f64) * t153061 - F::cast_from(0.53351469956445347664e-1_f64) * t153063 * t150331;
    (t153054, t153060, t153063, t153066)
}
