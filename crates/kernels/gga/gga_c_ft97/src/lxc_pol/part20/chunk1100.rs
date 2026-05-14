//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1100/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1100<F: Float>(t3771: F, t6813: F, t93053: F, t27671: F, t35415: F, t6808: F, t6809: F, t96535: F, t37481: F, t6789: F, t6793: F, t108479: F, t108950: F, t108965: F, t108969: F, t108972: F, t108977: F, t108981: F, t108983: F, t13402: F, t13407: F, t13443: F, t17807: F, t213: F, t218: F, t232: F, t2378: F, t2395: F, t2418: F, t24361: F, t2455: F, t25057: F, t27605: F, t27609: F, t27616: F, t27618: F, t27619: F, t27658: F, t27711: F, t27712: F, t3690: F, t3759: F, t3817: F, t66121: F, t66556: F, t709: F, t96660: F, t96695: F) -> (F,) {
    let t108992 = t3771 * t6813 * t93053;
    let t108998 = t27671 * t35415;
    let t109002 = t6808 * t96535 * t6809;
    let t109008 = t37481 * t6789 * t6793;
    let t109012 = 0.38731446812548799881e-3 * t3759 * t108950 * t13402 - 0.88910709717637694816e-2 * t13443 * t25057 * t218 * t3817 * t709 + 0.44455354858818847408e-2 * t27711 * t25057 * t27712 * t2455 - 0.93019603785751168e-1 * t96660 * t66121 + 0.2370952259137005195e-1 * t108965 * t66556 + 0.30274029503828221194e-3 * t27658 * t108969 + 0.3404992446913580247e-1 * t24361 * t108972 * t3690 * t108479 + 0.44540303667943584666e-3 * t27609 * t232 * t108977 - 0.29673063867321838427e-4 * t108981 * t232 * t108983 + 0.52801466802079540469e-5 * t27616 * t27618 * t27619 * t2418 - 0.87941772264679191251e-7 * t108992 * t27618 * t213 * t2378 * t2395 + 0.10357803499222879255e-4 * t96695 * t108998 + 0.85124811172839506173e-2 * t109002 + 0.61277550024922479209e-6 * t17807 * t27605 * t13407 - 0.10205883805138882776e-7 * t17807 * t109008 * t13402;
    (t109012,)
}
