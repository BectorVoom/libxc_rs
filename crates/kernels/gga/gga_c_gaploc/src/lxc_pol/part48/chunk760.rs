//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 760/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk760<F: Float>(t44939: F, t723: F, t13507: F, t7137: F, t13535: F, t7129: F, t2508: F, t2717: F, t3603: F, t10668: F, t13498: F, t13672: F, t1897: F, t2580: F, t2936: F, t3451: F, t43127: F, t43139: F, t44921: F, t44924: F, t44927: F, t44928: F, t44931: F, t44933: F, t44936: F, t44938: F, t702: F, t8637: F) -> (F, F) {
    let t44940 = t44939 * t723;
    let t44956 = 0.61524209841137794268e-1 * t7137 * t13507;
    let t44960 = 0.76905262301422242837e-2 * t7129 * t13535;
    let t44963 = 0.76905262301422242837e-2 * t2508 * t2717 * t3603;
    let t44964 = -t44921 + t44924 - t44927 + 0.12817543716903707139e-2 * t44928 + t44931 - t44933 - t44936 + t44938 + 0.15381052460284448567e-1 * t2508 * t2580 * t44940 - 0.76905262301422242837e-2 * t1897 * t13672 * t702 - 0.46143157380853345702e-1 * t7129 * t13498 - 0.46143157380853345702e-1 * t2508 * t8637 * t3451 - 0.46143157380853345702e-1 * t2508 * t2936 * t10668 - t44956 + 0.1281754371690370714e-2 * t43127 + 0.17090058289204942853e-2 * t43139 + t44960 + t44963;
    (t44940, t44964)
}
