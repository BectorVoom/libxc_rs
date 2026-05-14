//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1103/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1103<F: Float>(t17839: F, t6032: F, t17836: F, t24361: F, t24378: F, t27638: F, t1095: F, t2394: F, t2379: F, t6776: F, t695: F, t3758: F, t108650: F, t108660: F, t108925: F, t13426: F, t13443: F, t13449: F, t13453: F, t13522: F, t13651: F, t13666: F, t1417: F, t1701: F, t232: F, t2388: F, t2389: F, t24265: F, t24346: F, t24372: F, t2455: F, t25057: F, t27487: F, t27494: F, t27605: F, t27723: F, t6027: F, t6034: F, t66076: F, t6785: F, t96660: F) -> (F, F) {
    let t109063 = t6032 * t17839;
    let t109064 = t17836 * t109063;
    let t109069 = t24361 * t24378 * t27638;
    let t109080 = t2394 * t1095;
    let t109084 = t2379 * t1095;
    let t109108 = t695 * t6776;
    let t109109 = t3758 * t109108;
    let t109112 = -0.12255510004984495842e-5 * t66076 * t27605 * t13522 - 0.10338048737805743097e-3 * t109064 * t6785 * t13522 + 0.85124811172839506174e-2 * t109069 - 0.89080607335887169332e-3 * t24265 * t232 * t108925 - 0.89080607335887169332e-4 * t6034 * t232 * t108650 - 0.29673063867321838428e-4 * t24372 * t232 * t108660 - 0.93019603785751168e-2 * t24346 * t109080 * t2388 - 0.77462893625097599762e-3 * t24346 * t109084 * t2388 - 0.44455354858818847408e-2 * t13443 * t25057 * t27723 * t2455 - 0.11854761295685025975e-1 * t1417 * t1701 * t27494 * t2455 - 0.46509801892875584e-1 * t96660 * t13666 + 0.46509801892875584e-1 * t24346 * t13426 + 0.22227677429409423704e-2 * t1417 * t1701 * t6027 * t13651 + 0.93019603785751168e-2 * t27487 * t13449 + 0.46509801892875584e-2 * t27487 * t13453 + 0.46509801892875584e-1 * t109109 * t2389;
    (t109063, t109112)
}
