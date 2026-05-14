//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 980/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk980<F: Float>(t200: F, t2428: F, t2393: F, t3750: F, t703: F, t8715: F, t2455: F, t1154: F, t2514: F, t13827: F, t761: F, t265: F, t42163: F, t2469: F, t737: F, t2486: F) -> (F, F, F, F, F, F, F, F, F) {
    let t52595 = t200 * t2428;
    let t52608 = t2393 * t3750;
    let t52679 = t8715 * t703;
    let t52900 = t200 * t2455;
    let t53307 = t1154 * t2514;
    let t53484 = t13827 * t761;
    let t53504 = t42163 * t265;
    let t53513 = t737 * t2469;
    let t53642 = t2486 * t2469;
    (t52595, t52608, t52679, t52900, t53307, t53484, t53504, t53513, t53642)
}
