//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1196/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1196<F: Float>(t31097: F, t761: F, t766: F, t1882: F, t31241: F, t18216: F, t96834: F, t18749: F, t24412: F, t1175: F, t13839: F, t13885: F, t14127: F, t14175: F, t18491: F, t18641: F, t18646: F, t18713: F, t1901: F, t242: F, t24737: F, t24793: F, t2574: F, t27836: F, t27841: F, t28128: F, t28140: F, t28356: F, t30933: F, t31014: F, t3859: F, t446: F, t52002: F, t6074: F, t67996: F, t684: F, t6848: F, t6921: F, t773: F) -> (F, F, F, F) {
    let t122117 = t31097 * t761;
    let t122118 = t122117 * t766;
    let t122122 = t1882 * t31241;
    let t122162 = t96834 * t18216;
    let t122166 = t24412 * t18749;
    let t122173 = -t446 * t242 * t122118 / 3.0 - 2.0 / 9.0 * t122122 + 4.0 / 3.0 * t446 * t2574 * t1175 * t27836 + 4.0 / 3.0 * t446 * t2574 * t1175 * t27841 + 4.0 / 3.0 * t446 * t2574 * t773 * t31014 - 4.0 / 9.0 * t1901 * t14175 * t30933 * t684 - 2.0 * t1901 * t28140 * t6074 * t18641 - 2.0 / 3.0 * t1901 * t13885 * t24737 * t18646 + 2.0 / 9.0 * t1901 * t52002 * t6848 - 2.0 / 3.0 * t1901 * t14127 * t28128 * t18491 + 8.0 / 3.0 * t1901 * t67996 * t6921 * t3859 + 2.0 / 9.0 * t1901 * t24793 * t18713 - 2.0 * t446 * t242 * t122162 + 4.0 / 3.0 * t446 * t242 * t122166 + 2.0 / 9.0 * t1901 * t13839 * t28356;
    (t122118, t122162, t122166, t122173)
}
