//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1107/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1107<F: Float>(t147270: F, t23842: F, t147278: F, t23832: F, t137007: F, t147256: F, t554: F, t104732: F, t12374: F, t138773: F, t138825: F, t138839: F, t138843: F, t138867: F, t138870: F, t138961: F, t145154: F, t145376: F, t145382: F, t147262: F, t147266: F, t147274: F, t147291: F, t147310: F, t23711: F, t26714: F, t32140: F, t32763: F, t34907: F, t378: F, t7335: F, t94514: F, t94535: F) -> (F, F) {
    let t147331 = t23842 * t147270;
    let t147337 = t23832 * t147278;
    let t147344 = t137007 * t147256 * t554;
    let t147357 = F::cast_from(0.6041940442683716741e-1_f64) * t94535 * t147262 - F::cast_from(0.6041940442683716741e-1_f64) * t23711 * t145154 + F::cast_from(0.6041940442683716741e-1_f64) * t138825 - F::cast_from(0.6041940442683716741e-1_f64) * t138839 + F::cast_from(0.17783823318815115888e-1_f64) * t138843 + F::cast_from(0.80027204934668021496e-1_f64) * t12374 * t32763 * t34907 + F::cast_from(0.6041940442683716741e-1_f64) * t147331 - F::cast_from(0.14500657062440920178e1_f64) * t23842 * t147266 + F::cast_from(0.14500657062440920178e1_f64) * t23832 * t147274 - F::cast_from(0.6041940442683716741e-1_f64) * t147337 - F::cast_from(0.24008161480400406449e0_f64) * t138773 * t32140 * t378 * t26714 + F::cast_from(0.41054213886971219988e0_f64) * t7335 * t147344 - F::cast_from(0.19592980390298668092e-1_f64) * t138867 * t145382 + F::cast_from(0.19592980390298668092e-1_f64) * t138870 * t145382 - F::cast_from(0.14125722719362779755e-1_f64) * t138961 * t145376 + F::cast_from(0.36251642656102300446e0_f64) * t94514 * t147310 - F::cast_from(0.54377463984153450669e0_f64) * t104732 * t147291;
    (t147344, t147357)
}
