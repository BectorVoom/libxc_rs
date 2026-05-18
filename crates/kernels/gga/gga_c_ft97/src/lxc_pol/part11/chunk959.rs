//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 959/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk959<F: Float>(t1655: F, t2022: F, t535: F, t7977: F, t1580: F, t8965: F, t120: F, t1557: F, t1559: F, t126: F, t1595: F, t1631: F, t1736: F, t2021: F, t36377: F, t378: F, t37996: F, t38111: F, t38211: F, t39563: F, t39861: F, t39866: F, t39869: F, t39872: F, t39877: F, t39889: F, t534: F, t7914: F, t8693: F, t8942: F, t8948: F, t8963: F, t8964: F, t8977: F, t8994: F) -> F {
    let t39892 = t2022 * t1655;
    let t39895 = t535 * t7977;
    let t39907 = t8965 * t1580;
    let t39912 = t120 * t1557;
    let t39913 = t39912 * t1559;
    let t39917 = F::new(0.959348966341294683e-1) * t2021 * t39861 + F::new(0.105346283539551678e1) * t39563 * t8942 - F::new(0.15647667480826127546e-2) * t36377 * t39866 - F::new(0.46820570462022968e0) * t8693 * t39869 + F::new(0.68769182700451188138e-1) * t37996 * t39872 + F::new(0.32991033661753008702e-2) * t39877 * t39872 - F::new(0.532971647967385935e-1) * t534 * t38111 * t126 + F::new(0.24033558310605691936e-3) * t38211 * t39872 + F::new(0.41932428475884870816e-1) * t1631 * t39861 + F::new(0.13302972333265952938e0) * t39889 * t39872 - F::new(0.30699166922921429856e0) * t8977 * t39892 + F::new(0.1279131955121726244e0) * t2021 * t39895 - F::new(0.15095674251318553494e0) * t7914 * t39892 + F::new(0.55909904634513161088e-1) * t1631 * t39895 - F::new(0.5498505610292168117e-2) * t8994 * t39892 - F::new(0.49254336522043865661e-4) * t8948 * t378 * t39866 - F::new(0.8854768453090786061e-3) * t8963 * t8964 * t39907 - F::new(0.11806357937454381415e-2) * t8963 * t1736 * t1595 * t39913;
    t39917
}
