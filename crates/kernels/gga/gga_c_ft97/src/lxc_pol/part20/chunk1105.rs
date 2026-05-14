//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1105/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1105<F: Float>(t24345: F, t27703: F, t24286: F, t6828: F, t1417: F, t236: F, t6776: F, t13580: F, t108977: F, t108983: F, t109024: F, t1113: F, t13509: F, t13520: F, t13531: F, t13577: F, t17987: F, t2035: F, t231: F, t2389: F, t24311: F, t24346: F, t2455: F, t27527: F, t27546: F, t27566: F, t27700: F, t52594: F, t6023: F, t6045: F, t65763: F, t66066: F, t66071: F, t66088: F, t6979: F, t9543: F) -> (F, F) {
    let t109159 = t27703 * t24345;
    let t109168 = t6828 * t24286;
    let t109169 = t1417 * t109168;
    let t109200 = t236 * t6776;
    let t109201 = t13580 * t109200;
    let t109204 = 0.46509801892875584e-1 * t109159 * t2389 - 0.38731446812548799881e-3 * t24346 * t13509 + 0.52700762016626893448e-4 * t17987 * t2035 * t6979 * t2455 - 0.1134997482304526749e-1 * t109169 + 0.13519760450715832853e-3 * t13577 * t27700 - 0.67552196935353456646e-5 * t13531 * t27700 + 0.51690243689028715487e-4 * t27527 * t6023 * t108977 - 0.3443640424494650102e-5 * t27527 * t24311 * t108983 - 0.51690243689028715488e-4 * t13520 * t6023 * t65763 - 0.25845121844514357744e-4 * t13520 * t6023 * t66066 + 0.1721820212247325051e-5 * t13520 * t24311 * t66071 + 0.76612330055555555556e-1 * t27546 * t6045 * t231 * t1113 * t2455 + 0.27568129967481981592e-4 * t27566 * t66088 + 0.51690243689028715488e-4 * t52594 * t6023 * t109024 + 0.13519760450715832853e-3 * t9543 * t109201;
    (t109168, t109204)
}
