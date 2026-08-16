//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1189/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1189(t27856: f64, t7696: f64, t1245: f64, t27807: f64, t291: f64, t27812: f64, t283: f64, t5168: f64, t27876: f64, t2822: f64, t359: f64, t92807: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95877 = 0.12356481481481481482e-2_f64 * t7696 * t27856;
    let t95890 = t1245 * t291 * t27807;
    let t95892 = 0.12378114784505208333e-4_f64 * t27812 * t95890;
    let t95893 = t5168 * t283;
    let t95903 = t2822 * t27876;
    let t95911 = t92807 * t359;
    (t95877, t95890, t95892, t95893, t95903, t95911)
}
