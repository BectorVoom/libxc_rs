//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1063/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1063<F: Float>(t14443: F, t27821: F, t7703: F, t14570: F, t283: F, t990: F, t9588: F, t5025: F, t27779: F, t93435: F, t26685: F, t27825: F, t14447: F, t27949: F, t1245: F, t27774: F, t2909: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t95606 = t14443 * t27821;
    let t95608 = 0.15445601851851851852e-3 * t7703 * t95606;
    let t95640 = t14570 * t283 * t990;
    let t95655 = t9588 * t283;
    let t95664 = t5025 * t283;
    let t95684 = t93435 * t27779;
    let t95686 = 0.61836467013888888889e-4 * t26685 * t95684;
    let t95696 = 0.30891203703703703704e-3 * t7703 * t14443 * t27825;
    let t95698 = 0.20612155671296296296e-4 * t26685 * t95606;
    let t95764 = t7703 * t14447 * t27949;
    let t95775 = t7703 * t1245 * t2909 * t27774;
    (t95608, t95640, t95655, t95664, t95684, t95686, t95696, t95698, t95764, t95775)
}
