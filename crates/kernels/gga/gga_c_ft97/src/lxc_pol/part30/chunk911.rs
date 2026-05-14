//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 911/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk911<F: Float>(t2427: F, t6793: F, t224: F, t2344: F, t7205: F, t123619: F, t5009: F, t694: F, t109216: F, t3766: F, t6: F, t150319: F, t420: F, t108447: F, t1127: F, t123028: F, t142832: F, t150496: F, t150752: F, t27521: F, t27616: F, t27638: F, t27647: F, t27672: F, t33372: F, t33374: F, t33375: F, t33403: F, t33404: F, t33418: F, t35462: F, t3789: F, t6037: F, t6828: F, t709: F, t7464: F, t7470: F, t7477: F, t92354: F) -> (F, F) {
    let t150843 = t2427 * t6793;
    let t150844 = t224 * t150843;
    let t150845 = t7205 * t2344;
    let t150846 = t150845 * t123619;
    let t150849 = t694 * t5009;
    let t150858 = t3766 * t109216 * t6;
    let t150864 = t150319 * t420;
    let t150875 = 0.39525571512470170088e-4 * t7477 * t142832 * t7464 * t1127 * t709 - 0.68116566383613497688e-3 * t27521 * t7470 * t150496 - 0.13784064983740990796e-3 * t33418 * t150752 + 0.28200083969358461043e-4 * t150844 * t150846 - 0.1443087735596363459e-7 * t3789 * t150849 * t35462 * t709 - 0.40859909362962962964e0 * t33372 * t6828 * t33374 + 0.15322466011111111111e0 * t150858 * t33375 - 0.22705522127871165896e-3 * t108447 * t33404 * t27638 + 0.31680880081247724282e-4 * t27616 * t150864 * t6037 + 0.23022991505793434254e-7 * t123028 * t92354 * t33403 * t27672 - 0.22705522127871165896e-3 * t108447 * t33404 * t27647;
    (t150846, t150875)
}
