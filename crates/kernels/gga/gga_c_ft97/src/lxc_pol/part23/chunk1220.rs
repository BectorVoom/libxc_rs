//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1220/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1220<F: Float>(t18514: F, t6035: F, t9657: F, t27671: F, t420: F, t70290: F, t19177: F, t1096: F, t213: F, t688: F, t24361: F, t24378: F, t30660: F, t24372: F, t30656: F, t108525: F, t108823: F, t108838: F, t109216: F, t109266: F, t109273: F, t122770: F, t122774: F, t122889: F, t13411: F, t13520: F, t17828: F, t17836: F, t17859: F, t2387: F, t24260: F, t24311: F, t24389: F, t27500: F, t27506: F, t27527: F, t27529: F, t27546: F, t27548: F, t27552: F, t27584: F, t27669: F, t27670: F, t27692: F, t27704: F, t27707: F, t27733: F, t37481: F, t3766: F, t3774: F, t6023: F, t6817: F, t79485: F, t92354: F, t96537: F) -> (F, F, F) {
    let t123061 = t6035 * t9657 * t18514;
    let t123069 = t27671 * t420 * t70290;
    let t123074 = t27671 * t420 * t19177;
    let t123087 = t1096 * t213 * t688;
    let t123103 = t24361 * t24378 * t30660;
    let t123106 = t24372 * t24378 * t30656;
    let t123122 = 0.51074886703703703704e-1 * t27500 * t123061 - 0.46509801892875584e-1 * t27704 * t27692 - 0.98978452595430188147e-4 * t108823 - 0.85124811172839506173e-2 * t108838 - 0.51789017496114396277e-5 * t27670 * t123069 + 0.25876656037945937584e-6 * t122889 * t27669 * t123074 - 0.51690243689028715488e-5 * t3774 * t6023 * t122770 - 0.1721820212247325051e-5 * t3774 * t24311 * t122774 - 0.40859909362962962964e0 * t27546 * t27506 * t27548 - 0.21120586720831816188e-5 * t108525 * t109266 * t123087 - 0.17588354452935838251e-6 * t2387 * t92354 * t37481 * t6817 * t109273 * t123087 + 0.14187468528806584362e-2 * t96537 + 0.1836608226397146721e-4 * t17836 * t24389 * t13411 * t17828 + 0.85124811172839506173e-2 * t123103 - 0.4945510644553639738e-5 * t123106 - 0.55136259934963963185e-3 * t27527 * t27584 * t27529 + 0.27568129967481981593e-3 * t13520 * t27584 * t27552 + 8.0 * t27733 * t27707 + 8.0 * t3766 * t109216 * t17859 + 4.0 * t3766 * t24260 * t79485;
    (t123061, t123074, t123122)
}
