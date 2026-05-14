//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1195/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1195<F: Float>(t218: F, t4088: F, t25057: F, t820: F, t2719: F, t28628: F, t703: F, t1471: F, t19116: F, t112082: F, t112111: F, t112244: F, t112248: F, t14721: F, t14742: F, t14766: F, t25070: F, t28595: F, t28639: F, t28695: F, t4094: F, t4099: F, t6035: F, t684: F) -> (F,) {
    let t112282 = t218 * t4088;
    let t112284 = t25057 * t112282 * t820;
    let t112288 = t25057 * t28628 * t2719;
    let t112295 = t703 * t4088;
    let t112300 = t19116 * t1471;
    let t112311 = -0.90613700826057446696e0 * t14721 * t112248 + 0.90613700826057446696e0 * t14766 * t112284 + 0.45306850413028723348e0 * t14766 * t112288 - 0.45306850413028723348e0 * t14721 * t112288 + 0.12081826776807659559e1 * t4099 * t112082 - 0.66678001092592592594e-1 * t25070 * t6035 * t112295 * t684 - 0.90613700826057446696e0 * t112300 * t28639 - 0.90613700826057446696e0 * t14721 * t112284 + 0.45306850413028723348e0 * t14742 * t112244 - 0.90613700826057446696e0 * t28695 * t28595 + 0.45306850413028723348e0 * t4094 * t112111;
    (t112311,)
}
