//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1249/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1249<F: Float>(t33788: F, t33813: F, t33848: F, t33880: F, t33900: F, t33933: F, t33953: F, t33972: F, t15087: F, t15094: F, t32523: F, t33619: F, t33620: F, t33622: F, t33623: F, t33624: F, t33627: F, t33632: F, t33638: F, t33750: F, t33757: F, t4530: F, t4535: F, t555: F, t6607: F, t6638: F, t9557: F, t9882: F, t9891: F) -> (F, F) {
    let t33975 = t33788 + t33813 + t33848 + t33880 + t33900 + t33933 + t33953 + t33972;
    let t33977 = 2.0 * t15087 * t9882 - 6.0 * t15094 * t33757 + 2.0 * t32523 * t6607 + 2.0 * t33750 * t4535 + t33975 * t555 - t4530 * t9891 - t6638 * t9557 + t33619 + t33620 - t33622 + t33623 + t33624 + t33627 + t33632 - t33638;
    (t33975, t33977)
}
