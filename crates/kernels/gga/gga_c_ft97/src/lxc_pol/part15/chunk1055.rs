//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1055/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1055<F: Float>(t126: F, t85413: F, t85424: F, t120: F, t16832: F, t20049: F, t2014: F, t2021: F, t20592: F, t20606: F, t3359: F, t36377: F, t378: F, t37996: F, t38211: F, t39889: F, t4466: F, t4686: F, t72: F, t76899: F, t76914: F, t76918: F, t76926: F, t76928: F, t86741: F, t86744: F, t86747: F, t86750: F, t8948: F, t8963: F, t8994: F, t929: F) -> (F, F, F, F) {
    let t86753 = t85413 * t126;
    let t86756 = t85424 * t126;
    let t86763 = t85413 * t120;
    let t86771 = -F::new(0.44273842265453930305e-2) * t8963 * t76918 * t20606 + F::new(0.48229216329983294636e-3) * t8963 * t76914 * t20606 - F::new(0.59031789687271907074e-3) * t76899 + F::new(0.48229216329983294636e-3) * t76926 - F::new(0.44273842265453930305e-2) * t76928 - F::new(0.10625722143708943273e-1) * t2014 * t72 * t20049 * t929 * t120 + F::new(0.19923229019454268637e-2) * t8948 * t378 * t4686 * t4466 - F::new(0.5498505610292168117e-2) * t8994 * t86741 - F::new(0.438942848081465325e0) * t86744 * t120 - F::new(0.35115427846517226e0) * t86747 * t120 + F::new(0.23410285231011484e0) * t86750 * t120 + F::new(0.13302972333265952938e0) * t39889 * t86753 + F::new(0.959348966341294683e-1) * t2021 * t86756 + F::new(0.24033558310605691936e-3) * t38211 * t86753 + F::new(0.105346283539551678e1) * t16832 * t20592 - F::new(0.15647667480826127546e-2) * t36377 * t86763 - F::new(0.46820570462022968e0) * t3359 * t120 * t20049 + F::new(0.68769182700451188138e-1) * t37996 * t86753;
    (t86753, t86756, t86763, t86771)
}
