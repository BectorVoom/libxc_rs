//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 959/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk959(t1655: f64, t2022: f64, t535: f64, t7977: f64, t1580: f64, t8965: f64, t120: f64, t1557: f64, t1559: f64, t126: f64, t1595: f64, t1631: f64, t1736: f64, t2021: f64, t36377: f64, t378: f64, t37996: f64, t38111: f64, t38211: f64, t39563: f64, t39861: f64, t39866: f64, t39869: f64, t39872: f64, t39877: f64, t39889: f64, t534: f64, t7914: f64, t8693: f64, t8942: f64, t8948: f64, t8963: f64, t8964: f64, t8977: f64, t8994: f64) -> f64 {
    let t39892 = t2022 * t1655;
    let t39895 = t535 * t7977;
    let t39907 = t8965 * t1580;
    let t39912 = t120 * t1557;
    let t39913 = t39912 * t1559;
    let t39917 = 0.959348966341294683e-1_f64 * t2021 * t39861 + 0.105346283539551678e1_f64 * t39563 * t8942 - 0.15647667480826127546e-2_f64 * t36377 * t39866 - 0.46820570462022968e0_f64 * t8693 * t39869 + 0.68769182700451188138e-1_f64 * t37996 * t39872 + 0.32991033661753008702e-2_f64 * t39877 * t39872 - 0.532971647967385935e-1_f64 * t534 * t38111 * t126 + 0.24033558310605691936e-3_f64 * t38211 * t39872 + 0.41932428475884870816e-1_f64 * t1631 * t39861 + 0.13302972333265952938e0_f64 * t39889 * t39872 - 0.30699166922921429856e0_f64 * t8977 * t39892 + 0.1279131955121726244e0_f64 * t2021 * t39895 - 0.15095674251318553494e0_f64 * t7914 * t39892 + 0.55909904634513161088e-1_f64 * t1631 * t39895 - 0.5498505610292168117e-2_f64 * t8994 * t39892 - 0.49254336522043865661e-4_f64 * t8948 * t378 * t39866 - 0.8854768453090786061e-3_f64 * t8963 * t8964 * t39907 - 0.11806357937454381415e-2_f64 * t8963 * t1736 * t1595 * t39913;
    t39917
}
