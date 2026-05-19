//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1175/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1175<F: Float>(t43207: F, t88503: F, t2697: F, t88393: F, t274: F, t278: F, t801: F, t88773: F, t18826: F, t2014: F, t21373: F, t22096: F, t22110: F, t231: F, t2394: F, t2710: F, t36827: F, t4068: F, t41621: F, t41673: F, t43712: F, t43742: F, t683: F, t807: F, t83049: F, t83109: F, t8948: F, t8963: F) -> (F, F, F, F) {
    let t89947 = t43207 * t88503;
    let t89950 = t2697 * t88393;
    let t89958 = t88503 * t274;
    let t89964 = t88503 * t278;
    let t89972 = t88393 * t278;
    let t89981 = t801 * t88773;
    let t89994 = -F::cast_from(0.36171912247487470976e-3_f64) * t2014 * t231 * t88503 * t2697 * t274 - F::cast_from(0.438942848081465325e0_f64) * t89947 * t274 - F::cast_from(0.35115427846517226e0_f64) * t89950 * t274 + F::cast_from(0.48229216329983294636e-3_f64) * t8963 * t83109 * t22110 + F::cast_from(0.105346283539551678e1_f64) * t18826 * t22096 - F::cast_from(0.15647667480826127546e-2_f64) * t36827 * t89958 - F::cast_from(0.46820570462022968e0_f64) * t4068 * t274 * t21373 + F::cast_from(0.68769182700451188138e-1_f64) * t41673 * t89964 + F::cast_from(0.32991033661753008702e-2_f64) * t43742 * t89964 - F::cast_from(0.532971647967385935e-1_f64) * t807 * t88773 * t278 + F::cast_from(0.41932428475884870816e-1_f64) * t2394 * t89972 + F::cast_from(0.13302972333265952938e0_f64) * t43712 * t89964 + F::cast_from(0.959348966341294683e-1_f64) * t2710 * t89972 + F::cast_from(0.24033558310605691936e-3_f64) * t41621 * t89964 + F::cast_from(0.23410285231011484e0_f64) * t89981 * t274 - F::cast_from(0.79692916077817074549e-2_f64) * t2014 * t231 * t88393 * t274 - F::cast_from(0.49254336522043865661e-4_f64) * t8948 * t683 * t89958 - F::cast_from(0.44273842265453930305e-2_f64) * t8963 * t83049 * t22110;
    (t89947, t89950, t89981, t89994)
}
