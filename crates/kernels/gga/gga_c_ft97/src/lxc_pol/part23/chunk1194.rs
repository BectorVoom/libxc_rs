//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1194/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1194<F: Float>(t24526: F, t5165: F, t109925: F, t109936: F, t110751: F, t14163: F, t1456: F, t18515: F, t18641: F, t18676: F, t18708: F, t1901: F, t24747: F, t24793: F, t2574: F, t2599: F, t27753: F, t27757: F, t27763: F, t28378: F, t3821: F, t446: F, t4969: F, t5147: F, t53923: F, t6061: F, t66735: F, t68003: F, t68135: F, t6940: F, t729: F, t762: F, t97269: F) -> (F, F) {
    let t122055 = t24526 * t5165;
    let t122077 = -2.0 / 9.0 * t1901 * t2599 * t24747 * t4969 + t1901 * t24793 * t18708 / 9.0 + 2.0 / 3.0 * t446 * t729 * t762 * t6940 * t3821 + 2.0 / 3.0 * t446 * t2574 * t1456 * t18641 - t109925 - 4.0 / 27.0 * t97269 + 16.0 / 27.0 * t109936 + t446 * t729 * t762 * t6061 * t5147 / 3.0 - 4.0 / 9.0 * t1901 * t14163 * t122055 - 4.0 / 9.0 * t1901 * t68135 * t27753 - 2.0 / 9.0 * t1901 * t53923 * t28378 - 4.0 / 9.0 * t1901 * t66735 * t27757 + 4.0 / 27.0 * t1901 * t68003 * t27763 - 2.0 / 3.0 * t1901 * t24793 * t18515 - 4.0 / 3.0 * t1901 * t110751 * t18676;
    (t122055, t122077)
}
