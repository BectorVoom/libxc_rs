//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1039/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1039<F: Float>(t108447: F, t1127: F, t123028: F, t142832: F, t150496: F, t150752: F, t150844: F, t150846: F, t150849: F, t150858: F, t150864: F, t27521: F, t27616: F, t27638: F, t27647: F, t27672: F, t33372: F, t33374: F, t33375: F, t33403: F, t33404: F, t33418: F, t35462: F, t3789: F, t6037: F, t6828: F, t709: F, t7464: F, t7470: F, t7477: F, t92354: F) -> F {
    let t150875 = F::cast_from(0.39525571512470170088e-4_f64) * t7477 * t142832 * t7464 * t1127 * t709 - F::cast_from(0.68116566383613497688e-3_f64) * t27521 * t7470 * t150496 - F::cast_from(0.13784064983740990796e-3_f64) * t33418 * t150752 + F::cast_from(0.28200083969358461043e-4_f64) * t150844 * t150846 - F::cast_from(0.1443087735596363459e-7_f64) * t3789 * t150849 * t35462 * t709 - F::cast_from(0.40859909362962962964e0_f64) * t33372 * t6828 * t33374 + F::cast_from(0.15322466011111111111e0_f64) * t150858 * t33375 - F::cast_from(0.22705522127871165896e-3_f64) * t108447 * t33404 * t27638 + F::cast_from(0.31680880081247724282e-4_f64) * t27616 * t150864 * t6037 + F::cast_from(0.23022991505793434254e-7_f64) * t123028 * t92354 * t33403 * t27672 - F::cast_from(0.22705522127871165896e-3_f64) * t108447 * t33404 * t27647;
    t150875
}
