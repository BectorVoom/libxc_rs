//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1055/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1055(t126: f64, t85413: f64, t85424: f64, t120: f64, t16832: f64, t20049: f64, t2014: f64, t2021: f64, t20592: f64, t20606: f64, t3359: f64, t36377: f64, t378: f64, t37996: f64, t38211: f64, t39889: f64, t4466: f64, t4686: f64, t72: f64, t76899: f64, t76914: f64, t76918: f64, t76926: f64, t76928: f64, t86741: f64, t86744: f64, t86747: f64, t86750: f64, t8948: f64, t8963: f64, t8994: f64, t929: f64) -> (f64, f64, f64, f64) {
    let t86753 = t85413 * t126;
    let t86756 = t85424 * t126;
    let t86763 = t85413 * t120;
    let t86771 = -0.44273842265453930305e-2_f64 * t8963 * t76918 * t20606 + 0.48229216329983294636e-3_f64 * t8963 * t76914 * t20606 - 0.59031789687271907074e-3_f64 * t76899 + 0.48229216329983294636e-3_f64 * t76926 - 0.44273842265453930305e-2_f64 * t76928 - 0.10625722143708943273e-1_f64 * t2014 * t72 * t20049 * t929 * t120 + 0.19923229019454268637e-2_f64 * t8948 * t378 * t4686 * t4466 - 0.5498505610292168117e-2_f64 * t8994 * t86741 - 0.438942848081465325e0_f64 * t86744 * t120 - 0.35115427846517226e0_f64 * t86747 * t120 + 0.23410285231011484e0_f64 * t86750 * t120 + 0.13302972333265952938e0_f64 * t39889 * t86753 + 0.959348966341294683e-1_f64 * t2021 * t86756 + 0.24033558310605691936e-3_f64 * t38211 * t86753 + 0.105346283539551678e1_f64 * t16832 * t20592 - 0.15647667480826127546e-2_f64 * t36377 * t86763 - 0.46820570462022968e0_f64 * t3359 * t120 * t20049 + 0.68769182700451188138e-1_f64 * t37996 * t86753;
    (t86753, t86756, t86763, t86771)
}
