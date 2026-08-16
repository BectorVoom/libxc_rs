//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1175/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1175(t43207: f64, t88503: f64, t2697: f64, t88393: f64, t274: f64, t278: f64, t801: f64, t88773: f64, t18826: f64, t2014: f64, t21373: f64, t22096: f64, t22110: f64, t231: f64, t2394: f64, t2710: f64, t36827: f64, t4068: f64, t41621: f64, t41673: f64, t43712: f64, t43742: f64, t683: f64, t807: f64, t83049: f64, t83109: f64, t8948: f64, t8963: f64) -> (f64, f64, f64, f64) {
    let t89947 = t43207 * t88503;
    let t89950 = t2697 * t88393;
    let t89958 = t88503 * t274;
    let t89964 = t88503 * t278;
    let t89972 = t88393 * t278;
    let t89981 = t801 * t88773;
    let t89994 = -0.36171912247487470976e-3_f64 * t2014 * t231 * t88503 * t2697 * t274 - 0.438942848081465325e0_f64 * t89947 * t274 - 0.35115427846517226e0_f64 * t89950 * t274 + 0.48229216329983294636e-3_f64 * t8963 * t83109 * t22110 + 0.105346283539551678e1_f64 * t18826 * t22096 - 0.15647667480826127546e-2_f64 * t36827 * t89958 - 0.46820570462022968e0_f64 * t4068 * t274 * t21373 + 0.68769182700451188138e-1_f64 * t41673 * t89964 + 0.32991033661753008702e-2_f64 * t43742 * t89964 - 0.532971647967385935e-1_f64 * t807 * t88773 * t278 + 0.41932428475884870816e-1_f64 * t2394 * t89972 + 0.13302972333265952938e0_f64 * t43712 * t89964 + 0.959348966341294683e-1_f64 * t2710 * t89972 + 0.24033558310605691936e-3_f64 * t41621 * t89964 + 0.23410285231011484e0_f64 * t89981 * t274 - 0.79692916077817074549e-2_f64 * t2014 * t231 * t88393 * t274 - 0.49254336522043865661e-4_f64 * t8948 * t683 * t89958 - 0.44273842265453930305e-2_f64 * t8963 * t83049 * t22110;
    (t89947, t89950, t89981, t89994)
}
