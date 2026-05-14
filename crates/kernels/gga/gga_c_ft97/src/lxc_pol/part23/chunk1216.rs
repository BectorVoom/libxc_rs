//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1216/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1216<F: Float>(t6033: F, t79402: F, t108517: F, t108524: F, t108597: F, t108618: F, t108640: F, t108761: F, t108797: F, t108810: F, t108817: F, t1091: F, t122858: F, t122869: F, t122874: F, t122889: F, t122895: F, t122899: F, t122904: F, t122909: F, t17807: F, t17818: F, t17864: F, t24260: F, t24276: F, t24278: F, t24361: F, t27540: F, t27620: F, t27637: F, t27651: F, t30763: F, t35410: F, t35456: F, t3746: F, t3762: F, t3766: F, t3773: F, t3774: F, t3789: F, t4939: F, t4978: F, t6027: F, t6035: F, t6038: F, t684: F, t709: F, t79601: F, t79605: F, t96510: F) -> (F,) {
    let t122915 = t79402 * t6033;
    let t122918 = t108517 * t108524;
    let t122925 = 0.85124811172839506172e-2 * t108597 + 0.38482339615903025572e-7 * t3789 * t122858 * t30763 * t709 - 12.0 * t3766 * t96510 * t79601 + 8.0 * t3766 * t24260 * t79605 - 0.12768721675925925926e-1 * t27651 * t6035 * t122869 * t684 + 0.12768721675925925926e-1 * t24361 * t6035 * t122874 * t684 - 0.17024962234567901235e-1 * t108618 + 0.20182686335885480796e-3 * t108640 + 0.25537443351851851852e-1 * t24361 * t6035 * t108761 * t1091 - 0.51074886703703703704e-1 * t24361 * t108817 * t27637 * t3746 + 0.30030568862539529421e-7 * t122889 * t17818 * t3773 * t6027 * t4939 + 0.87299078230359608375e-3 * t3774 * t122895 * t3762 - 0.10417183504236821465e-4 * t17807 * t122899 * t3762 - 0.14846767889314528222e-4 * t122904 - 0.3959138103817207526e-3 * t108797 * t27540 + 0.49489226297715094073e-4 * t122909 + 0.7423383944657264111e-4 * t24276 * t24278 * t4978 * t684 + 0.44540303667943584666e-3 * t122915 * t6038 + 0.21120586720831816187e-4 * t122918 * t35410 * t27620 - 0.21120586720831816187e-4 * t108810 * t35456 * t17864;
    (t122925,)
}
