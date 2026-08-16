//! GGA_C_PBE lxc pol — lxc_pol chunk-first struct-interface chunk 2/5.
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[derive(CubeType)]
pub struct Chunk2Out<F: Float> {
    pub t2196: F,
    pub t2202: F,
    pub t2204: F,
    pub t2206: F,
    pub t2208: F,
    pub t2215: F,
    pub t2217: F,
    pub t2218: F,
    pub t2221: F,
    pub t2223: F,
    pub t2224: F,
    pub t2229: F,
    pub t2244: F,
    pub t2247: F,
    pub t2248: F,
    pub t2251: F,
    pub t2254: F,
    pub t2257: F,
    pub t2258: F,
    pub t2260: F,
    pub t2265: F,
    pub t2267: F,
    pub t2270: F,
    pub t2272: F,
    pub t2279: F,
    pub t2280: F,
    pub t2292: F,
    pub t2296: F,
    pub t2300: F,
    pub t2303: F,
    pub t2304: F,
    pub t2306: F,
    pub t2316: F,
    pub t2317: F,
    pub t2325: F,
    pub t2330: F,
    pub t2331: F,
    pub t2334: F,
    pub t2335: F,
    pub t2338: F,
    pub t2356: F,
    pub t2359: F,
    pub t2366: F,
    pub t2384: F,
    pub t2391: F,
    pub t2392: F,
    pub t2394: F,
    pub t2395: F,
    pub t2396: F,
    pub t2397: F,
    pub t2399: F,
    pub t2402: F,
    pub t2405: F,
    pub t2406: F,
    pub t2409: F,
    pub t2411: F,
    pub t2412: F,
    pub t2413: F,
    pub t2414: F,
    pub t2417: F,
    pub t2420: F,
    pub t2424: F,
    pub t2425: F,
    pub t2426: F,
    pub t2429: F,
    pub t2432: F,
    pub t2434: F,
    pub t2435: F,
    pub t2438: F,
    pub t2440: F,
    pub t2442: F,
    pub t2443: F,
    pub t2446: F,
    pub t2460: F,
    pub t2461: F,
    pub t2464: F,
    pub t2472: F,
    pub t2473: F,
    pub t2476: F,
    pub t2477: F,
    pub t2480: F,
    pub t2483: F,
    pub t2485: F,
    pub t2486: F,
    pub t2489: F,
    pub t2491: F,
    pub t2498: F,
    pub t2500: F,
    pub t2503: F,
    pub t2506: F,
    pub t2508: F,
    pub t2509: F,
    pub t2510: F,
    pub t2512: F,
    pub t2513: F,
    pub t2515: F,
    pub t2517: F,
    pub t2521: F,
    pub t2526: F,
    pub t2531: F,
    pub t2536: F,
    pub t2541: F,
    pub t2544: F,
    pub t2550: F,
    pub t2551: F,
    pub t2552: F,
    pub t2553: F,
    pub t2554: F,
    pub t2555: F,
    pub t2560: F,
    pub t2567: F,
    pub t2572: F,
    pub t2580: F,
    pub t2581: F,
    pub t2582: F,
    pub t2585: F,
    pub t2587: F,
    pub t2591: F,
    pub t2593: F,
    pub t2596: F,
    pub t2598: F,
    pub t2601: F,
    pub t2603: F,
    pub t2613: F,
    pub t2619: F,
    pub t2631: F,
    pub t2632: F,
    pub t2635: F,
    pub t2640: F,
    pub t2641: F,
    pub t2644: F,
    pub t2647: F,
    pub t2648: F,
    pub t2650: F,
    pub t2654: F,
    pub t2657: F,
    pub t2659: F,
    pub t2669: F,
    pub t2670: F,
    pub t2678: F,
    pub t2682: F,
    pub t2686: F,
    pub t2690: F,
    pub t2697: F,
    pub t2701: F,
    pub t2705: F,
    pub t2709: F,
    pub t2711: F,
    pub t2715: F,
    pub t2720: F,
    pub t2722: F,
    pub t2725: F,
    pub t2727: F,
    pub t2730: F,
    pub t2732: F,
    pub t2737: F,
    pub t2741: F,
    pub t2745: F,
    pub t2749: F,
    pub t2752: F,
    pub t2754: F,
    pub t2758: F,
    pub t2761: F,
    pub t2763: F,
    pub t2774: F,
    pub t2775: F,
    pub t2780: F,
    pub t2787: F,
    pub t2788: F,
    pub t2791: F,
    pub t2795: F,
    pub t2796: F,
    pub t2802: F,
    pub t2815: F,
    pub t2819: F,
    pub t2829: F,
    pub t2832: F,
    pub t2849: F,
    pub t2850: F,
    pub t2852: F,
    pub t2853: F,
    pub t2854: F,
    pub t2855: F,
    pub t2856: F,
    pub t2859: F,
    pub t2870: F,
    pub t2871: F,
    pub t2872: F,
    pub t2878: F,
    pub t2883: F,
    pub t2888: F,
    pub t2894: F,
    pub t2895: F,
    pub t2896: F,
    pub t2898: F,
    pub t2899: F,
    pub t2900: F,
    pub t2901: F,
    pub t2902: F,
    pub t2904: F,
    pub t2905: F,
    pub t2907: F,
    pub t2908: F,
    pub t2912: F,
    pub t2913: F,
    pub t2914: F,
    pub t2915: F,
    pub t2916: F,
    pub t2917: F,
    pub t2920: F,
    pub t2921: F,
    pub t2922: F,
    pub t2923: F,
    pub t2924: F,
    pub t2925: F,
    pub t2926: F,
    pub t2928: F,
    pub t2929: F,
    pub t2930: F,
    pub t2931: F,
    pub t2932: F,
    pub t2933: F,
    pub t2935: F,
    pub t2937: F,
    pub t2938: F,
    pub t2941: F,
    pub t2959: F,
    pub t2960: F,
    pub t2961: F,
    pub t2962: F,
    pub t2964: F,
    pub t2967: F,
    pub t2968: F,
    pub t2970: F,
    pub t2973: F,
    pub t2975: F,
    pub t2980: F,
    pub t2986: F,
    pub t2988: F,
    pub t2995: F,
    pub t2999: F,
    pub t3007: F,
    pub t3010: F,
    pub t3013: F,
    pub t3016: F,
    pub t3017: F,
    pub t3019: F,
    pub t3022: F,
    pub t3024: F,
    pub t3030: F,
    pub t3034: F,
    pub t3036: F,
    pub t3040: F,
    pub t3043: F,
    pub t3045: F,
    pub t3049: F,
    pub t3052: F,
    pub t3054: F,
    pub t3057: F,
    pub t3059: F,
    pub t3063: F,
    pub t3066: F,
    pub t3067: F,
    pub t3075: F,
    pub t3076: F,
    pub t3080: F,
    pub t3081: F,
    pub t3089: F,
    pub t3090: F,
    pub t3093: F,
    pub t3096: F,
    pub t3100: F,
    pub t3110: F,
    pub t3114: F,
    pub t3121: F,
    pub t3131: F,
    pub t3132: F,
    pub t3134: F,
    pub t3135: F,
    pub t3136: F,
    pub t3154: F,
    pub t3159: F,
    pub t3160: F,
    pub t3163: F,
    pub t3166: F,
    pub t3170: F,
    pub t3173: F,
    pub t3177: F,
    pub t3185: F,
    pub t3187: F,
    pub t3191: F,
    pub t3195: F,
    pub t3205: F,
    pub t3212: F,
    pub t3213: F,
    pub t3220: F,
    pub t3221: F,
    pub t3225: F,
    pub t3230: F,
    pub t3231: F,
    pub t3232: F,
    pub t3235: F,
    pub t3236: F,
    pub t3238: F,
    pub t3239: F,
    pub t3242: F,
    pub t3243: F,
    pub t3246: F,
    pub t3247: F,
    pub t3250: F,
    pub t3251: F,
    pub t3254: F,
    pub t3255: F,
    pub t3258: F,
    pub t3260: F,
    pub t3262: F,
    pub t3264: F,
    pub t3268: F,
    pub t3269: F,
    pub t3272: F,
    pub t3273: F,
    pub t3276: F,
    pub t3280: F,
    pub t3285: F,
    pub t3288: F,
    pub t3294: F,
    pub t3310: F,
    pub t3311: F,
    pub t3326: F,
    pub t3329: F,
    pub t3332: F,
    pub t3333: F,
    pub t3337: F,
    pub t3340: F,
    pub t3341: F,
    pub t3344: F,
    pub t3345: F,
    pub t3348: F,
    pub t3350: F,
    pub t3352: F,
    pub t3354: F,
    pub t3358: F,
    pub t3359: F,
    pub t3362: F,
    pub t3363: F,
    pub t3366: F,
    pub t3374: F,
    pub t3377: F,
    pub t3381: F,
    pub t3384: F,
    pub t3386: F,
    pub t3392: F,
    pub t3393: F,
    pub t3394: F,
    pub t3400: F,
    pub t3421: F,
    pub t3422: F,
    pub t3426: F,
    pub t3430: F,
    pub t3431: F,
    pub t3435: F,
    pub t3440: F,
    pub t3441: F,
    pub t3442: F,
    pub t3443: F,
    pub t3444: F,
    pub t3448: F,
    pub t3449: F,
    pub t3453: F,
    pub t3454: F,
    pub t3457: F,
    pub t3458: F,
    pub t3462: F,
    pub t3463: F,
    pub t3466: F,
    pub t3467: F,
    pub t3470: F,
    pub t3472: F,
    pub t3474: F,
    pub t3475: F,
    pub t3478: F,
    pub t3479: F,
    pub t3482: F,
    pub t3483: F,
    pub t3485: F,
    pub t3488: F,
    pub t3492: F,
    pub t3493: F,
    pub t3496: F,
    pub t3500: F,
    pub t3502: F,
    pub t3506: F,
    pub t3514: F,
    pub t3520: F,
    pub t3523: F,
    pub t3525: F,
    pub t3540: F,
    pub t3541: F,
    pub t3545: F,
    pub t3556: F,
    pub t3557: F,
    pub t3560: F,
    pub t3564: F,
    pub t3565: F,
    pub t3569: F,
    pub t3573: F,
    pub t3576: F,
    pub t3579: F,
    pub t3581: F,
    pub t3583: F,
    pub t3586: F,
    pub t3587: F,
    pub t3590: F,
    pub t3591: F,
    pub t3593: F,
    pub t3596: F,
    pub t3600: F,
    pub t3601: F,
    pub t3607: F,
    pub t3612: F,
    pub t3617: F,
    pub t3620: F,
    pub t3623: F,
    pub t3627: F,
    pub t3646: F,
    pub t3647: F,
    pub t3654: F,
    pub t3655: F,
    pub t3663: F,
    pub t3664: F,
    pub t3668: F,
    pub t3669: F,
    pub t3672: F,
    pub t3673: F,
    pub t3676: F,
    pub t3677: F,
    pub t3680: F,
    pub t3683: F,
    pub t3686: F,
    pub t3688: F,
    pub t3690: F,
    pub t3692: F,
    pub t3696: F,
    pub t3699: F,
    pub t3702: F,
    pub t3707: F,
    pub t3710: F,
    pub t3716: F,
    pub t3730: F,
    pub t3731: F,
    pub t3746: F,
    pub t3749: F,
    pub t3752: F,
    pub t3753: F,
    pub t3757: F,
    pub t3760: F,
    pub t3763: F,
    pub t3766: F,
    pub t3768: F,
    pub t3770: F,
    pub t3772: F,
    pub t3776: F,
    pub t3779: F,
    pub t3782: F,
    pub t3784: F,
    pub t3785: F,
    pub t3786: F,
    pub t3789: F,
    pub t3790: F,
    pub t3791: F,
    pub t3792: F,
    pub t3793: F,
    pub t3795: F,
    pub t3796: F,
    pub t3800: F,
    pub t3801: F,
    pub t3804: F,
    pub t3806: F,
    pub t3807: F,
    pub t3809: F,
    pub t3811: F,
    pub t3812: F,
    pub t3814: F,
    pub t3815: F,
    pub t3818: F,
    pub t3819: F,
    pub t3822: F,
    pub t3825: F,
    pub t3826: F,
    pub t3829: F,
    pub t3830: F,
    pub t3831: F,
    pub t3834: F,
    pub t3835: F,
    pub t3836: F,
    pub t3837: F,
    pub t3840: F,
    pub t3841: F,
    pub t3844: F,
    pub t3845: F,
    pub t3847: F,
    pub t3848: F,
    pub t3849: F,
    pub t3850: F,
    pub t3851: F,
    pub t3852: F,
    pub t3854: F,
    pub t3855: F,
    pub t3856: F,
    pub t3857: F,
    pub t3858: F,
    pub t3860: F,
    pub tv3rho31: F,
    pub tv3rho32: F,
    pub tv3rho33: F,
    pub tv3rho2sigma0: F,
    pub tv3rho2sigma1: F,
    pub tv3rho2sigma2: F,
    pub tv3rho2sigma3: F,
    pub tv3rho2sigma4: F,
    pub tv3rho2sigma5: F,
    pub tv3rho2sigma6: F,
    pub tv3rho2sigma7: F,
    pub tv3rho2sigma8: F,
}

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_lxc_pol_chunk2<F: Float>(t43: F, t50: F, t1597: F, t1599: F, t1383: F, t1386: F, t1390: F, t1406: F, t1432: F, t1557: F, t1559: F, t1564: F, t2155: F, t2156: F, t2188: F, t2190: F, t2193: F, t2194: F, t1604: F, t1609: F, t1632: F, t185: F, t873: F, t190: F, t871: F, t350: F, t704: F, t1479: F, t1499: F, t1602: F, t1615: F, t1624: F, t1627: F, t1630: F, t1636: F, t1774: F, t1775: F, t1782: F, t1784: F, t1786: F, t349: F, t682: F, t40: F, t1793: F, t1803: F, t2186: F, t85: F, t4: F, t550: F, t1523: F, t1780: F, t1791: F, t1798: F, t1802: F, t1808: F, t116: F, t2154: F, t361: F, t560: F, t745: F, t312: F, t831: F, t262: F, t579: F, t893: F, t133: F, t2124: F, t118: F, t119: F, t290: F, t292: F, t370: F, t372: F, t712: F, t716: F, t719: F, t887: F, t894: F, t897: F, t121: F, t287: F, t288: F, t741: F, t921: F, t747: F, t1715: F, t366: F, t590: F, t833: F, t5: F, t275: F, t1705: F, t1707: F, t2132: F, t2133: F, t2135: F, t2138: F, t274: F, t285: F, t310: F, t1748: F, t849: F, t907: F, t2042: F, t622: F, t624: F, t1756: F, t840: F, t296: F, t839: F, t375: F, t609: F, t295: F, t2081: F, t1716: F, t1718: F, t1749: F, t1765: F, t1860: F, t1864: F, t1866: F, t620: F, t837: F, t2047: F, t2129: F, t320: F, t1884: F, t393: F, t764: F, t762: F, t954: F, t331: F, t784: F, t934: F, t756: F, t319: F, t900: F, t141: F, t385: F, t608: F, t722: F, t123: F, t143: F, t1742: F, t1894: F, t2029: F, t325: F, t388: F, t610: F, t723: F, t728: F, t768: F, t841: F, t928: F, t941: F, t944: F, t324: F, t142: F, t148: F, t332: F, t386: F, t394: F, t757: F, t765: F, t785: F, t929: F, t933: F, t935: F, t955: F, t335: F, t101: F, t792: F, t957: F, t334: F, t252: F, t793: F, t1362: F, t1365: F, t1368: F, t1527: F, t1535: F, t1996: F, t1998: F, t2000: F, t2002: F, t2140: F, t2143: F, t2145: F, t2147: F, t2150: F, t2151: F, t2152: F, t2153: F, t151: F, t397: F, t559: F, t958: F, t396: F, t1936: F, t788: F, t967: F, t362: F, t1618: F, t790: F, t787: F, t960: F, t1976: F, t1977: F, t1991: F, t1992: F, t1993: F, t1994: F, t686: F, t690: F, t7: F, t794: F, t797: F, t1968: F, t1971: F, t1987: F, t563: F, t582: F, t688: F, t697: F, t706: F, t709: F, t789: F, t991: F, t1332: F, t1333: F, t1334: F, t1954: F, t1955: F, t1958: F, t1960: F, t1961: F, t1962: F, t1981: F, t1982: F, t1984: F, t1985: F, t511: F, t544: F, t802: F, t805: F, t874: F, t881: F, t987: F, t1995: F, t1014: F, t75: F, t249: F, t2139: F, t1058: F, t2144: F, t2146: F, t1539: F, t1545: F, t1547: F, t1549: F, t1: F, t244: F, t1336: F, t1121: F, t336: F, t992: F, t2189: F, t1017: F, t1034: F, t1033: F, t993: F, t1120: F, t1566: F, t996: F, t1000: F, t512: F, t1573: F, t516: F, t195: F, t47: F, t517: F, t853: F, t1005: F, t1581: F, t1008: F, t524: F, t199: F, t52: F, t861: F, t59: F, t87: F, t237: F, t1641: F, t564: F, t253: F, t814: F, t1653: F, t571: F, t257: F, t822: F, t1018: F, t1049: F, t1859: F, t1673: F, t1063: F, t1067: F, t1070: F, t901: F, t1046: F, t6: F, t1684: F, t1693: F, t2018: F, t2026: F, t2074: F, t1039: F, t1706: F, t1043: F, t1090: F, t1086: F, t314: F, t1038: F, t365: F, t1042: F, t1710: F, t594: F, t2075: F, t376: F, t919: F, t1053: F, t2082: F, t2076: F, t1073: F, t1055: F, t1842: F, t1725: F, t2028: F, t1737: F, t1076: F, t616: F, t1081: F, t1098: F, t1117: F, t1093: F, t1102: F, t1106: F, t1110: F, t1094: F, t1099: F, t1118: F, t1016: F, t1019: F, t1036: F, t1059: F, t1122: F, t995: F, t689: F, t691: F, t695: F, t1997: F, t340: F, t344: F, t1553: F, t1562: F, t1598: F, t1605: F, t1610: F, t1840: F, t607: F, t1885: F, t986: F, t1015: F, t1035: F, t1967: F, t1969: F, t1970: F, t879: F, t994: F, t1129: F, t1170: F, t1176: F, t401: F, t98: F, t1126: F, t56: F, t585: F, t108: F, t404: F, t1132: F, t277: F, t402: F, t596: F, t1136: F, t600: F, t103: F, t604: F, t129: F, t280: F, t612: F, t1139: F, t302: F, t298: F, t131: F, t626: F, t1141: F, t725: F, t730: F, t408: F, t737: F, t1146: F, t316: F, t1148: F, t749: F, t753: F, t414: F, t417: F, t1155: F, t1163: F, t411: F, t415: F, t1165: F, t1151: F, t1158: F, t282: F, t419: F, t1152: F, t1156: F, t1160: F, t1164: F, t1166: F, t1168: F, t412: F, t418: F, t420: F, t422: F, t1175: F, t100: F, t1128: F, t1174: F, t1180: F, t1202: F, t1207: F, t1178: F, t427: F, t1197: F, t1188: F, t1189: F, t1194: F, t1198: F, t1200: F, t428: F, t430: F, t432: F, t1206: F, t1209: F, t1233: F, t19: F, t111: F, t843: F, t378: F, t903: F, t914: F, t382: F, t925: F, t1222: F, t1219: F, t1223: F, t1226: F, t1218: F, t1229: F, t1231: F, t1237: F, t1239: F, t1258: F, t1246: F, t1251: F, t1245: F, t1254: F, t1256: F, t1262: F, t1047: F, t1074: F, t1079: F, t1291: F, t1264: F, t130: F, t147: F, t1265: F, t1269: F, t323: F, t1273: F, t1286: F, t1277: F, t1280: F, t1272: F, t1278: F, t1282: F, t1287: F, t138: F, t1281: F, t136: F, t134: F, t1288: F, t305: F, param_BB: F, param_beta: F, param_gamma: F, zeta_threshold: F) -> Chunk2Out<F> {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t2195 = F::cast_from(12.0_f64) * t1597;
    let t2196 = F::cast_from(24.0_f64) * t1599;
    let t2197 = t2155 + t2156 - t1557 + t1559 + t1383 - t1386 - t1390 + t1406 + t1432 + t2188 + t2190 - t2193 - t2194 + t1564 - t2195 - t2196;
    let t2199 = F::cast_from(48.0_f64) * t1604;
    let t2200 = F::cast_from(80.0_f64) * t1609;
    let t2201 = F::cast_from(0.21687162600603479684e-1_f64) * t1632;
    let t2202 = t185 * t873;
    let t2203 = F::cast_from(8.0_f64) * t2202;
    let t2204 = t190 * t873;
    let t2205 = F::cast_from(8.0_f64) * t2204;
    let t2206 = t185 * t871;
    let t2207 = F::cast_from(8.0_f64) * t2206;
    let t2208 = t704 * t350;
    let t2209 = F::cast_from(12.0_f64) * t2208;
    let t2210 = -t1602 + t2199 + t2200 - t1615 - t1479 - t1624 + t1627 + t1630 + t2201 - t1636 + t1499 + t2203 - t2205 + t2207 + t2209 - t1774;
    let t2211 = F::cast_from(0.11696447245269292414e1_f64) * t1775;
    let t2212 = F::cast_from(0.5848223622634646207e0_f64) * t1782;
    let t2213 = F::cast_from(0.34631718211362927518e2_f64) * t1784;
    let t2214 = F::cast_from(0.23392894490538584828e1_f64) * t1786;
    let t2215 = t190 * t871;
    let t2216 = F::cast_from(8.0_f64) * t2215;
    let t2217 = t349 * t682;
    let t2218 = t40 * t2217;
    let t2219 = F::cast_from(0.18311447306006545054e-3_f64) * t1793;
    let t2220 = F::cast_from(0.4883052614935078681e-3_f64) * t1803;
    let t2221 = t2186 * t85;
    let t2222 = F::cast_from(0.19751673498613801407e-1_f64) * t2221;
    let t2223 = t349 * t4;
    let t2224 = t2223 * t550;
    let t2225 = F::cast_from(0.10843581300301739842e-1_f64) * t2224;
    let t2226 = -t2211 - t1780 - t2212 - t2213 + t2214 + t1791 - t2216 + t2218 - t2219 - t1798 + t1802 + t2220 - t1808 + t1523 + t2222 + t2225;
    let t2229 = (t2154 + t2197 + t2210 + t2226) * t116;
    let t2244 = t745 * t361 * t560;
    let t2247 = t312 * t831;
    let t2248 = t2247 * t262;
    let t2251 = t893 * t579;
    let t2254 = t133 * t2124;
    let t2257 = F::cast_from(60.0_f64) * t118 * t2244 - F::cast_from(24.0_f64) * t118 * t2248 - F::cast_from(12.0_f64) * t118 * t2251 + F::cast_from(3.0_f64) * t118 * t2254 - t2229 * t119 - F::cast_from(24.0_f64) * t290 * t894 + F::cast_from(6.0_f64) * t290 * t897 + F::cast_from(6.0_f64) * t887 * t292 - F::cast_from(12.0_f64) * t370 * t716 + F::cast_from(3.0_f64) * t370 * t719 + F::cast_from(3.0_f64) * t712 * t372;
    let t2258 = t2257 * t121;
    let t2260 = t287 * t288 * t2258;
    let t2264 = F::cast_from(35.0_f64) / F::cast_from(576.0_f64) * t741 * t921;
    let t2265 = t831 * t262;
    let t2267 = t747 * t288 * t2265;
    let t2270 = t361 * t579;
    let t2272 = t747 * t288 * t2270;
    let t2275 = t1715 * t366;
    let t2278 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t590 * t833;
    let t2279 = t5 * t2124;
    let t2280 = t275 * t2279;
    let t2284 = t2132 - F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t2133 - t1705 - F::cast_from(119.0_f64) / F::cast_from(13824.0_f64) * t2135 + t2138 - t285 * t2260 / F::cast_from(3072.0_f64) - t2264 + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t310 * t2267 + F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t310 * t2272 - F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t2275 + t2278 - t274 * t2280 / F::cast_from(48.0_f64) - F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t1707;
    let t2288 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t1748 * t849;
    let t2290 = F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t1748 * t907;
    let t2292 = t622 * t2042 * t624;
    let t2296 = t622 * t840 * t1756;
    let t2300 = t839 * t2042 * t296;
    let t2303 = t375 * t609;
    let t2304 = t295 * t262;
    let t2306 = t2081 * t2303 * t2304;
    let t2314 = -F::cast_from(35.0_f64) / F::cast_from(108.0_f64) * t1716 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t1718 - t2288 + t2290 + t620 * t2292 / F::cast_from(384.0_f64) + t620 * t2296 / F::cast_from(768.0_f64) - t620 * t2300 / F::cast_from(1536.0_f64) - t837 * t2306 / F::cast_from(192.0_f64) - F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t1749 + F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t1765 - F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t1860 - F::cast_from(119.0_f64) / F::cast_from(6912.0_f64) * t1864 + F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t1866;
    let t2316 = t2047 + t2129 + t2284 + t2314;
    let t2317 = param_beta * t2316;
    let t2325 = t320 * t116;
    let t2330 = t1884 * t393;
    let t2331 = t2330 * t764;
    let t2334 = t762 * t954;
    let t2335 = t2334 * t331;
    let t2338 = t934 * t784;
    let t2356 = t756 * t375;
    let t2359 = t319 * t900;
    let t2366 = t141 * t2257;
    let t2376 = t385 * t608;
    let t2384 = t385 * t722;
    let t2391 = -F::cast_from(2.0_f64) * t325 * t928 * t295 * t121 - t325 * t2356 * t121 - F::cast_from(2.0_f64) * t325 * t2359 * t121 - t325 * t2366 * t121 - t325 * t2376 * t121 - t325 * t2384 * t121 + t143 * t123 * t2316 + F::cast_from(2.0_f64) * t768 * t388 * t1742 - F::cast_from(6.0_f64) * t1894 * t388 * t2029 + F::cast_from(2.0_f64) * t768 * t2376 * t609 - F::cast_from(2.0_f64) * t325 * t941 * t296 - F::cast_from(2.0_f64) * t325 * t944 * t296 - t325 * t388 * t723 - t325 * t388 * t728 + F::cast_from(6.0_f64) * t768 * t388 * t610 + F::cast_from(4.0_f64) * t768 * t941 * t841 + F::cast_from(4.0_f64) * t768 * t944 * t841;
    let t2392 = t324 * t2391;
    let t2394 = -t142 * t2392 + t2317 * t148 + F::cast_from(4.0_f64) * t2325 * t935 - F::cast_from(6.0_f64) * t933 * t2331 + F::cast_from(4.0_f64) * t933 * t2335 + F::cast_from(2.0_f64) * t933 * t2338 - F::cast_from(2.0_f64) * t320 * t955 - F::cast_from(2.0_f64) * t929 * t332 + F::cast_from(2.0_f64) * t386 * t765 - t386 * t785 - t757 * t394;
    let t2395 = t2394 * t335;
    let t2396 = t101 * t2395;
    let t2397 = t957 * t792;
    let t2399 = t101 * t2397 * t334;
    let t2402 = t252 * t793 * t361;
    let t2404 = t1996 - t1998 + F::cast_from(6.0_f64) * t2000 + t2002 + t1527 + t2396 + t2140 + t2143 - F::cast_from(2.0_f64) * t2399 + t1362 - t1365 - t1535 - F::cast_from(3.0_f64) * t2402 - t2145 - t2147 - t2150 - t2151 - t2152 - t2153 + t1368;
    let t2405 = t151 * t2124;
    let t2406 = t252 * t2405;
    let t2409 = t252 * t397 * t579;
    let t2411 = t559 * t334;
    let t2412 = t335 * t361;
    let t2413 = t2412 * t262;
    let t2414 = t2411 * t2413;
    let t2417 = t559 * t397 * t560;
    let t2420 = t252 * t958 * t262;
    let t2422 = t2155 + t2156 + F::cast_from(3.0_f64) * t2406 - t1557 + t1559 + t1383 - t1386 - t1390 + t1406 + F::cast_from(3.0_f64) * t2409 + F::cast_from(12.0_f64) * t2414 + t1432 + F::cast_from(6.0_f64) * t2417 + F::cast_from(6.0_f64) * t2420 + t2188 + t2190 - t2193 - t2194 + t1564 - t2195;
    let t2424 = t252 * t396;
    let t2425 = t1936 * t262;
    let t2426 = t2424 * t2425;
    let t2429 = t252 * t788 * t361;
    let t2432 = t559 * t967 * t262;
    let t2434 = t362 * t579;
    let t2435 = t559 * t2434;
    let t2437 = -t2196 - t1602 + t2199 + t2200 - t1615 - F::cast_from(6.0_f64) * t2426 - t1479 - t1624 + t1627 + t1630 + t2201 - t1636 + t1499 + F::cast_from(3.0_f64) * t2429 + F::cast_from(12.0_f64) * t2432 + F::cast_from(6.0_f64) * t2435 + t2203 - t2205 + t2207 + t2209;
    let t2438 = t396 * t1618;
    let t2440 = t101 * t2438 * t790;
    let t2442 = param_gamma * t560;
    let t2443 = t2442 * t362;
    let t2446 = t101 * t960 * t787;
    let t2447 = -t1774 - t2211 - t1780 - t2212 - t2213 + F::cast_from(2.0_f64) * t2440 + F::cast_from(6.0_f64) * t2443 + t2214 + t1791 - t2216 + t2218 - t2219 - t1798 + t1802 + t2220 - t1808 + t1523 - t2446 + t2222 + t2225;
    let t2451 = -F::cast_from(16.0_f64) * t690 + t686 + t1991 - t1976 + t1977 - t794 + t797 + t1992 + t1993 + t1994 + t7 * (t2404 + t2422 + t2437 + t2447);
    let tv3rho31 = t1987 - t991 + t563 + t582 + t789 + t697 + t1968 - t706 - t709 - t1971 + F::cast_from(4.0_f64) * t688 + t2451;
    let t2457 = -F::cast_from(0.23392894490538584828e1_f64) * t802 + t1332 + t1333 - t1334 - F::cast_from(0.73245789224026180217e-3_f64) * t805 + t1981 - t1982 + t987 - t1954 - t1955 - t511 + t1984 + F::cast_from(4.0_f64) * t874 + t1985 - t1958 - t544 + t1960 + t1961 + t1962 - F::cast_from(16.0_f64) * t881;
    let t2459 = F::cast_from(40.0_f64) * t1995;
    let t2460 = t1014 * t75;
    let t2461 = t2460 * t249;
    let t2462 = F::cast_from(0.5848223622634646207e0_f64) * t2461;
    let t2463 = F::cast_from(0.23392894490538584828e1_f64) * t2139;
    let t2464 = t185 * t1058;
    let t2465 = F::cast_from(4.0_f64) * t2464;
    let t2466 = F::cast_from(0.11696447245269292414e1_f64) * t2144;
    let t2467 = F::cast_from(0.34631718211362927517e2_f64) * t2146;
    let t2468 = F::cast_from(8.0_f64) * t1539;
    let t2469 = F::cast_from(32.0_f64) * t1545;
    let t2470 = F::cast_from(20.0_f64) * t1547;
    let t2471 = F::cast_from(8.0_f64) * t1549;
    let t2472 = t1014 * t1;
    let t2473 = t2472 * t244;
    let t2474 = F::cast_from(0.18311447306006545054e-3_f64) * t2473;
    let t2475 = t2459 + t1336 + t1527 - t2462 + t2463 + t2465 + t2143 + t1362 - t1365 - t1535 - t2466 - t2467 - t2150 - t2468 + t1368 + t2469 + t2470 - t2471 - t2156 - t2474;
    let t2476 = t362 * t831;
    let t2477 = t559 * t2476;
    let t2480 = t252 * t1121 * t262;
    let t2483 = t252 * t958 * t361;
    let t2485 = t336 * t992;
    let t2486 = t559 * t2485;
    let t2488 = F::cast_from(0.48830526149350786811e-3_f64) * t2189;
    let t2489 = t1017 * t1618;
    let t2491 = t101 * t2489 * t334;
    let t2493 = F::cast_from(12.0_f64) * t1599;
    let t2494 = F::cast_from(12.0_f64) * t2477 - t1557 + t1383 - t1386 - t1390 + t1406 + t1432 + F::cast_from(3.0_f64) * t2480 + F::cast_from(6.0_f64) * t2483 + F::cast_from(6.0_f64) * t2486 + t2488 - t2193 + t2194 + t1564 + F::cast_from(2.0_f64) * t2491 - t2195 + t2493 + t1602 + t2199 - t2200;
    let t2496 = F::cast_from(0.10843581300301739842e-1_f64) * t1632;
    let t2497 = F::cast_from(16.0_f64) * t2204;
    let t2498 = F::cast_from(24.0_f64) * t2208;
    let t2500 = t101 * t960 * t957;
    let t2503 = t559 * t1034 * t262;
    let t2506 = t252 * t336 * t1033;
    let t2508 = t792 * t361;
    let t2509 = t2508 * t334;
    let t2510 = t2424 * t2509;
    let t2512 = param_gamma * t262;
    let t2513 = t2512 * t993;
    let t2515 = t1120 * t792;
    let t2517 = t101 * t2515 * t334;
    let t2518 = F::cast_from(0.5848223622634646207e0_f64) * t1775;
    let t2519 = -t1615 - t1479 - t1624 + t1627 + t1630 + t2496 - t1636 + t1499 - t2497 - t2207 - t2498 - F::cast_from(2.0_f64) * t2500 + F::cast_from(6.0_f64) * t2503 + F::cast_from(3.0_f64) * t2506 - F::cast_from(6.0_f64) * t2510 + F::cast_from(6.0_f64) * t2513 - t2517 - t1774 - t2518 - t1780;
    let t2520 = F::cast_from(0.17315859105681463759e2_f64) * t1784;
    let t2521 = t1566 * t996;
    let t2526 = t512 * t1000;
    let t2531 = -F::cast_from(2.0_f64) * t516 - F::cast_from(6.0_f64) * t1573;
    let t2535 = piecewise3::<F>(t44, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t2521 * t195 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t853 * t517 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2526 * t195 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t47 * t2531);
    let t2536 = t1581 * t1005;
    let t2541 = t524 * t1008;
    let t2544 = -t2531;
    let t2548 = piecewise3::<F>(t51, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t2536 * t199 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t861 * t517 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2541 * t199 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t52 * t2544);
    let t2550 = (t2535 + t2548) * t59;
    let t2551 = t2550 * t87;
    let t2552 = t40 * t2551;
    let t2553 = t1014 * t237;
    let t2554 = t40 * t2553;
    let t2555 = t1641 * t996;
    let t2560 = t564 * t1000;
    let t2566 = piecewise3::<F>(t44, F::cast_from(0.0_f64), F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t2555 * t195 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t814 * t517 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t2560 * t195 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t253 * t2531);
    let t2567 = t1653 * t1005;
    let t2572 = t571 * t1008;
    let t2578 = piecewise3::<F>(t51, F::cast_from(0.0_f64), F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t2567 * t199 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t822 * t517 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t2572 * t199 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t257 * t2544);
    let t2580 = t2566 / F::cast_from(2.0_f64) + t2578 / F::cast_from(2.0_f64);
    let t2581 = t151 * t2580;
    let t2582 = t252 * t2581;
    let t2585 = t252 * t1018 * t262;
    let t2587 = t190 * t1058;
    let t2588 = F::cast_from(4.0_f64) * t2587;
    let t2589 = t1859 * t1049;
    let t2591 = t361 * t831;
    let t2593 = t747 * t288 * t2591;
    let t2596 = t1033 * t262;
    let t2598 = t747 * t288 * t2596;
    let t2601 = t992 * t262;
    let t2603 = t1673 * t288 * t2601;
    let t2606 = t2459 + t1336 + t1527 - t2462 + t2463 + t2465 + t2143 + t1362 - t1365 - t1535 - t2466 - t2467 - t2150 - t2468 + t1368 + t2469;
    let t2607 = t2470 - t2471 - t2156 - t2474 - t1557 + t1383 - t1386 - t1390 + t1406 + t1432 + t2488 - t2193 + t2194 + t1564 - t2195 + t2493;
    let t2609 = t1602 + t2199 - t2200 - t1615 - t1479 - t1624 + t1627 + t1630 + t2496 - t1636 + t1499 - t2497 - t2207 - t2498 - t1774 - t2518;
    let t2610 = F::cast_from(0.11696447245269292414e1_f64) * t1786;
    let t2611 = F::cast_from(2.0_f64) * t2218;
    let t2612 = F::cast_from(0.24415263074675393405e-3_f64) * t1803;
    let t2613 = t2550 * t85;
    let t2614 = F::cast_from(0.19751673498613801407e-1_f64) * t2613;
    let t2615 = F::cast_from(0.21687162600603479684e-1_f64) * t2224;
    let t2616 = -t1780 - t2520 + t2552 + t2554 - t2588 + t2610 + t1791 - t2216 + t2611 - t1798 + t1802 + t2612 - t1808 + t1523 + t2614 + t2615;
    let t2619 = (t2606 + t2607 + t2609 + t2616) * t116;
    let t2631 = t745 * t992;
    let t2632 = t2631 * t262;
    let t2635 = t893 * t831;
    let t2640 = t312 * t1033;
    let t2641 = t2640 * t262;
    let t2644 = t133 * t2580;
    let t2647 = F::cast_from(3.0_f64) * t1063 * t292 - F::cast_from(12.0_f64) * t290 * t1067 + F::cast_from(3.0_f64) * t290 * t1070 + F::cast_from(60.0_f64) * t118 * t2632 - F::cast_from(24.0_f64) * t118 * t2635 - F::cast_from(12.0_f64) * t118 * t2641 + F::cast_from(3.0_f64) * t118 * t2644 - t2619 * t119 - F::cast_from(24.0_f64) * t370 * t894 + F::cast_from(6.0_f64) * t370 * t897 + F::cast_from(6.0_f64) * t887 * t372;
    let t2648 = t2647 * t121;
    let t2650 = t287 * t288 * t2648;
    let t2654 = t839 * t840 * t901;
    let t2657 = t6 * t1046;
    let t2659 = t839 * t2657 * t296;
    let t2663 = -t1684 - t2018 - t2026 - F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t2589 + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t310 * t2593 + F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t310 * t2598 - F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t310 * t2603 - t285 * t2650 / F::cast_from(3072.0_f64) - t2074 - t620 * t2654 / F::cast_from(1536.0_f64) - t620 * t2659 / F::cast_from(3072.0_f64) - F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t1693;
    let t2665 = t1706 * t1039;
    let t2667 = t590 * t1043;
    let t2669 = t5 * t2580;
    let t2670 = t275 * t2669;
    let t2673 = t741 * t1090;
    let t2675 = t741 * t1086;
    let t2678 = t314 * t288 * t2580;
    let t2682 = t275 * t1038 * t262;
    let t2686 = t275 * t365 * t831;
    let t2690 = t275 * t1042 * t262;
    let t2694 = t2132 - F::cast_from(119.0_f64) / F::cast_from(1728.0_f64) * t2133 - F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t2665 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t2667 - t274 * t2670 / F::cast_from(48.0_f64) + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t2673 - F::cast_from(35.0_f64) / F::cast_from(1152.0_f64) * t2675 - t310 * t2678 / F::cast_from(768.0_f64) - t1705 - t1710 * t2682 / F::cast_from(4.0_f64) + t594 * t2686 / F::cast_from(8.0_f64) + t594 * t2690 / F::cast_from(16.0_f64) - F::cast_from(119.0_f64) / F::cast_from(6912.0_f64) * t2135;
    let t2697 = t2075 * t376 * t919;
    let t2701 = t2081 * t2082 * t1053;
    let t2705 = t2081 * t2303 * t2076;
    let t2709 = t6 * t1073;
    let t2711 = t839 * t2709 * t841;
    let t2715 = t839 * t2657 * t841;
    let t2718 = t1748 * t1055;
    let t2720 = t1842 * t295;
    let t2722 = t839 * t2657 * t2720;
    let t2725 = t6 * t992;
    let t2727 = t1725 * t2725 * t296;
    let t2730 = t6 * t1033;
    let t2732 = t622 * t2730 * t296;
    let t2735 = t2138 - t2264 - F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t620 * t2697 + t620 * t2701 / F::cast_from(384.0_f64) - t837 * t2705 / F::cast_from(192.0_f64) - F::cast_from(35.0_f64) / F::cast_from(108.0_f64) * t2275 + t2278 + t837 * t2711 / F::cast_from(1536.0_f64) + t837 * t2715 / F::cast_from(512.0_f64) - F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t2718 - t2028 * t2722 / F::cast_from(512.0_f64) - F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t620 * t2727 + t620 * t2732 / F::cast_from(768.0_f64);
    let t2737 = t622 * t2709 * t624;
    let t2741 = t839 * t2709 * t296;
    let t2745 = t622 * t2657 * t624;
    let t2749 = t622 * t2042 * t1053;
    let t2752 = t121 * t831;
    let t2754 = t622 * t840 * t2752;
    let t2758 = t622 * t2657 * t1737;
    let t2761 = t609 * t900;
    let t2763 = t839 * t840 * t2761;
    let t2767 = t616 * t1076;
    let t2769 = t616 * t1081;
    let t2772 = t620 * t2737 / F::cast_from(768.0_f64) - t620 * t2741 / F::cast_from(3072.0_f64) + t620 * t2745 / F::cast_from(768.0_f64) + t620 * t2749 / F::cast_from(384.0_f64) + t620 * t2754 / F::cast_from(384.0_f64) - t837 * t2758 / F::cast_from(384.0_f64) + t837 * t2763 / F::cast_from(768.0_f64) - F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t1716 - t2288 + t2290 + F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t2767 + F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t2769 - F::cast_from(119.0_f64) / F::cast_from(13824.0_f64) * t1864;
    let t2774 = t2663 + t2694 + t2735 + t2772;
    let t2775 = param_beta * t2774;
    let t2780 = t386 * t116;
    let t2787 = t1884 * t1098;
    let t2788 = t2787 * t331;
    let t2791 = t934 * t954;
    let t2795 = t762 * t1117;
    let t2796 = t2795 * t331;
    let t2802 = t319 * t1046;
    let t2815 = t928 * t375;
    let t2819 = t385 * t900;
    let t2829 = t319 * t1073;
    let t2832 = t141 * t2647;
    let t2849 = -t325 * t1093 * t295 * t121 - F::cast_from(6.0_f64) * t1894 * t1102 * t2720 - t325 * t1102 * t296 + F::cast_from(6.0_f64) * t768 * t1102 * t841 - F::cast_from(2.0_f64) * t325 * t1106 * t296 + F::cast_from(4.0_f64) * t768 * t1106 * t841 - t325 * t1110 * t296 + F::cast_from(2.0_f64) * t768 * t1110 * t841 - t325 * t2802 * t121 - F::cast_from(2.0_f64) * t325 * t2815 * t121 - F::cast_from(2.0_f64) * t325 * t2819 * t121 - t325 * t2829 * t121 - t325 * t2832 * t121 + t143 * t123 * t2774 + F::cast_from(4.0_f64) * t768 * t388 * t2761 + F::cast_from(2.0_f64) * t768 * t2802 * t609 - F::cast_from(2.0_f64) * t325 * t388 * t901;
    let t2850 = t324 * t2849;
    let t2852 = -t1094 * t332 + F::cast_from(2.0_f64) * t320 * t1099 - t320 * t1118 - t142 * t2850 + t2775 * t148 + F::cast_from(4.0_f64) * t2780 * t935 - F::cast_from(6.0_f64) * t933 * t2788 + F::cast_from(4.0_f64) * t933 * t2791 + F::cast_from(2.0_f64) * t933 * t2796 - F::cast_from(2.0_f64) * t386 * t955 - F::cast_from(2.0_f64) * t929 * t394;
    let t2853 = t2852 * t335;
    let t2854 = t101 * t2853;
    let t2855 = t559 * t396;
    let t2856 = t2855 * t2413;
    let t2859 = t252 * t397 * t831;
    let t2861 = -t2520 + t2552 + t2554 + F::cast_from(3.0_f64) * t2582 - F::cast_from(3.0_f64) * t2585 - t2588 + t2854 + F::cast_from(12.0_f64) * t2856 + t2610 + t1791 + F::cast_from(6.0_f64) * t2859 - t2216 + t2611 - t1798 + t1802 + t2612 - t1808 + t1523 + t2614 + t2615;
    let t2865 = t1977 + t995 + t1016 - t1019 + t1036 + t1122 + t1059 + t1992 + t7 * (t2475 + t2494 + t2519 + t2861) + t1993 + t1994;
    let tv3rho32 = t2457 - t697 + t1968 - t706 + t709 - t1971 - t695 + t689 - t691 + t1991 - t1976 + t2865;
    let t2870 = F::cast_from(60.0_f64) * t1995;
    let t2871 = F::cast_from(96.0_f64) * t1997;
    let t2872 = t996 * t340;
    let t2878 = -F::cast_from(6.0_f64) * t516 - F::cast_from(6.0_f64) * t1573;
    let t2882 = piecewise3::<F>(t44, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t1566 * t2872 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t853 * t1000 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t47 * t2878);
    let t2883 = t1005 * t344;
    let t2888 = -t2878;
    let t2892 = piecewise3::<F>(t51, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t1581 * t2883 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t861 * t1008 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t52 * t2888);
    let t2894 = (t2882 + t2892) * t59;
    let t2895 = t2894 * t85;
    let t2896 = F::cast_from(0.19751673498613801407e-1_f64) * t2895;
    let t2897 = F::cast_from(0.17544670867903938621e1_f64) * t2461;
    let t2898 = t1017 * t396;
    let t2899 = t2898 * t1618;
    let t2900 = t101 * t2899;
    let t2901 = F::cast_from(2.0_f64) * t2900;
    let t2902 = F::cast_from(0.35089341735807877242e1_f64) * t2139;
    let t2904 = t101 * t960 * t1120;
    let t2905 = F::cast_from(3.0_f64) * t2904;
    let t2906 = F::cast_from(12.0_f64) * t2464;
    let t2907 = F::cast_from(0.17544670867903938621e1_f64) * t2144;
    let t2908 = F::cast_from(0.51947577317044391276e2_f64) * t2146;
    let t2909 = t2870 + t2871 + t2896 + t1527 - t2897 + t2901 + t2902 - t2905 - t2906 + t1362 - t1365 - t1535 - t2907 - t2908 + t1368;
    let t2910 = F::cast_from(0.54934341918019635162e-3_f64) * t2473;
    let t2912 = t252 * t397 * t1033;
    let t2913 = F::cast_from(9.0_f64) * t2912;
    let t2914 = t397 * t992;
    let t2915 = t559 * t2914;
    let t2916 = F::cast_from(18.0_f64) * t2915;
    let t2917 = F::cast_from(0.73245789224026180216e-3_f64) * t2189;
    let t2918 = -t1553 - t2910 + t2913 - t1557 + t1383 - t1386 - t1390 + t1406 + t1432 + t2916 + t2917 + t1562 + t1564 + t1598 - t1602;
    let t2920 = t992 * t361;
    let t2921 = param_gamma * t2920;
    let t2922 = t2921 * t151;
    let t2923 = F::cast_from(6.0_f64) * t2922;
    let t2924 = t1034 * t361;
    let t2925 = t559 * t2924;
    let t2926 = F::cast_from(18.0_f64) * t2925;
    let t2928 = t252 * t1018 * t361;
    let t2929 = F::cast_from(9.0_f64) * t2928;
    let t2930 = t2894 * t87;
    let t2931 = t40 * t2930;
    let t2932 = F::cast_from(24.0_f64) * t2202;
    let t2933 = F::cast_from(24.0_f64) * t2204;
    let t2934 = -t1605 - t1610 - t1615 + t2923 + t2926 - t1479 - t1624 + t1627 + t1630 - t1636 - t2929 + t1499 + t2931 - t2932 - t2933;
    let t2935 = F::cast_from(36.0_f64) * t2208;
    let t2937 = t252 * t1121 * t361;
    let t2938 = F::cast_from(9.0_f64) * t2937;
    let t2939 = F::cast_from(3.0_f64) * t2554;
    let t2940 = F::cast_from(12.0_f64) * t2587;
    let t2941 = F::cast_from(3.0_f64) * t2218;
    let t2949 = piecewise3::<F>(t44, F::cast_from(0.0_f64), F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t1641 * t2872 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t814 * t1000 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t253 * t2878);
    let t2957 = piecewise3::<F>(t51, F::cast_from(0.0_f64), F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t1653 * t2883 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t822 * t1008 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t257 * t2888);
    let t2959 = t2949 / F::cast_from(2.0_f64) + t2957 / F::cast_from(2.0_f64);
    let t2960 = t151 * t2959;
    let t2961 = t252 * t2960;
    let t2962 = F::cast_from(3.0_f64) * t2961;
    let t2964 = t275 * t365 * t1033;
    let t2967 = t1046 * t375;
    let t2968 = t2967 * t1842;
    let t2970 = t287 * t288 * t2968;
    let t2973 = t2967 * t609;
    let t2975 = t287 * t288 * t2973;
    let t2980 = t314 * t288 * t2959;
    let t2986 = t361 * t1033;
    let t2988 = t747 * t288 * t2986;
    let t2991 = t2870 + t2871 + t2896 + t1527 - t2897 + t2902 - t2906 + t1362 - t1365 - t1535 - t2907 - t2908;
    let t2992 = t1368 - t1553 - t2910 - t1557 + t1383 - t1386 - t1390 + t1406 + t1432 + t2917 + t1562 + t1564 + t1598;
    let t2994 = -t1602 - t1605 - t1610 - t1615 - t1479 - t1624 + t1627 + t1630 - t1636 + t1499 + t2931 - t2932;
    let t2995 = F::cast_from(0.32530743900905219526e-1_f64) * t2224;
    let t2996 = -t2933 + t2935 - t1774 - t1780 + t2939 - t2940 + t1791 + t2941 - t1798 + t1802 - t1808 + t1523 + t2995;
    let t2999 = (t2991 + t2992 + t2994 + t2996) * t116;
    let t3007 = t745 * t2920;
    let t3010 = t893 * t1033;
    let t3013 = t133 * t2959;
    let t3016 = F::cast_from(9.0_f64) * t1063 * t372 - F::cast_from(36.0_f64) * t370 * t1067 + F::cast_from(9.0_f64) * t370 * t1070 + F::cast_from(60.0_f64) * t118 * t3007 - F::cast_from(36.0_f64) * t118 * t3010 + F::cast_from(3.0_f64) * t118 * t3013 - t2999 * t119;
    let t3017 = t3016 * t121;
    let t3019 = t287 * t288 * t3017;
    let t3022 = t2967 * t121;
    let t3024 = t287 * t288 * t3022;
    let t3030 = t1673 * t288 * t2920;
    let t3033 = F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t594 * t2964 - t1840 * t2970 / F::cast_from(512.0_f64) + t607 * t2975 / F::cast_from(512.0_f64) - t1684 - F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t2589 - t310 * t2980 / F::cast_from(768.0_f64) - F::cast_from(119.0_f64) / F::cast_from(1152.0_f64) * t2133 - F::cast_from(7.0_f64) / F::cast_from(16.0_f64) * t2665 + F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t2667 + F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t310 * t2988 - t285 * t3019 / F::cast_from(3072.0_f64) - t285 * t3024 / F::cast_from(3072.0_f64) + F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t2673 - F::cast_from(35.0_f64) / F::cast_from(384.0_f64) * t2675 - F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t310 * t3030;
    let t3034 = t121 * t1033;
    let t3036 = t622 * t840 * t3034;
    let t3040 = t839 * t2709 * t376;
    let t3043 = t609 * t1073;
    let t3045 = t839 * t840 * t3043;
    let t3049 = t622 * t2709 * t1053;
    let t3052 = t609 * t361;
    let t3054 = t622 * t2657 * t3052;
    let t3057 = t121 * t992;
    let t3059 = t1725 * t840 * t3057;
    let t3063 = t622 * t2657 * t1053;
    let t3066 = t5 * t2959;
    let t3067 = t275 * t3066;
    let t3075 = t5 * t2920;
    let t3076 = t275 * t3075;
    let t3079 = -t1705 + t620 * t3036 / F::cast_from(256.0_f64) - t620 * t3040 / F::cast_from(1024.0_f64) + t837 * t3045 / F::cast_from(512.0_f64) + t620 * t3049 / F::cast_from(256.0_f64) - t837 * t3054 / F::cast_from(128.0_f64) - F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t620 * t3059 + t620 * t3063 / F::cast_from(256.0_f64) - t274 * t3067 / F::cast_from(48.0_f64) - F::cast_from(119.0_f64) / F::cast_from(4608.0_f64) * t2135 - F::cast_from(35.0_f64) / F::cast_from(72.0_f64) * t2275 - F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t2718 + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t2767 + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t2769 - t1710 * t3076 / F::cast_from(4.0_f64);
    let t3080 = t3033 + t3079;
    let t3081 = param_beta * t3080;
    let t3089 = t1098 * t393;
    let t3090 = t1885 * t3089;
    let t3093 = t934 * t1117;
    let t3096 = t141 * t2967;
    let t3100 = t385 * t1046;
    let t3110 = t1093 * t375;
    let t3114 = t385 * t1073;
    let t3121 = t141 * t3016;
    let t3131 = -F::cast_from(3.0_f64) * t325 * t1110 * t376 - t325 * t3096 * t121 - F::cast_from(3.0_f64) * t325 * t3100 * t121 - F::cast_from(3.0_f64) * t325 * t3110 * t121 - F::cast_from(3.0_f64) * t325 * t3114 * t121 - t325 * t3121 * t121 + t143 * t123 * t3080 - F::cast_from(6.0_f64) * t1894 * t3096 * t1842 + F::cast_from(6.0_f64) * t768 * t388 * t3043 + F::cast_from(6.0_f64) * t768 * t3096 * t609 + F::cast_from(6.0_f64) * t768 * t3100 * t609;
    let t3132 = t324 * t3131;
    let t3134 = -F::cast_from(3.0_f64) * t1094 * t394 + F::cast_from(6.0_f64) * t386 * t1099 - F::cast_from(3.0_f64) * t386 * t1118 - F::cast_from(6.0_f64) * t142 * t3090 - t142 * t3132 + t3081 * t148 + F::cast_from(6.0_f64) * t933 * t3093;
    let t3135 = t3134 * t335;
    let t3136 = t101 * t3135;
    let t3137 = t2935 + t2938 - t1774 - t1780 + t2939 - t2940 + t1791 + t2941 - t1798 + t1802 - t1808 + t1523 + t2962 + t3136 + t2995;
    let t3142 = -F::cast_from(0.35089341735807877242e1_f64) * t802 + t1332 + t1333 - t1334 - F::cast_from(0.10986868383603927032e-2_f64) * t805 + F::cast_from(18.0_f64) * t986 - t1954 - t1955 + t7 * (t2909 + t2918 + t2934 + t3137) + F::cast_from(6.0_f64) * t874 - t1958 + t1960 + t1961 + t1962;
    let t3151 = -F::cast_from(24.0_f64) * t879 - F::cast_from(24.0_f64) * t881 - t1967 + t1968 + t1969 + t1970 - t1971 - t1976 + t1977 + F::cast_from(18.0_f64) * t994 + F::cast_from(0.59255020495841404221e-1_f64) * t1015 - F::cast_from(3.0_f64) * t1019 + F::cast_from(9.0_f64) * t1035 + F::cast_from(3.0_f64) * t1122 + F::cast_from(3.0_f64) * t1059;
    let tv3rho33 = t3142 + t3151;
    let t3152 = t252 * t1129;
    let t3154 = t1170 * t335;
    let t3155 = t101 * t3154;
    let t3157 = t101 * t1176;
    let t3159 = t401 * t98;
    let t3160 = t1126 * t560;
    let t3163 = t3154 * t262;
    let t3166 = t262 * t334;
    let t3170 = t1126 * t579;
    let t3173 = t585 * t56;
    let t3175 = t3173 * t108 * t404;
    let t3176 = F::cast_from(35.0_f64) / F::cast_from(432.0_f64) * t3175;
    let t3177 = t1132 * t119;
    let t3178 = t3177 * t277;
    let t3179 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t3178;
    let t3180 = t402 * t133;
    let t3181 = t3180 * t596;
    let t3183 = t1136 * t600;
    let t3185 = t604 * t103;
    let t3187 = t280 * t3185 * t129;
    let t3188 = t3187 * t612;
    let t3191 = t280 * t1139 * t302;
    let t3192 = t3191 * t298;
    let t3193 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t3192;
    let t3195 = t280 * t1139 * t131;
    let t3196 = t3195 * t626;
    let t3198 = t1141 * t725;
    let t3200 = t1141 * t730;
    let t3202 = t408 * t737;
    let t3203 = F::cast_from(119.0_f64) / F::cast_from(6912.0_f64) * t3202;
    let t3205 = t280 * t1146 * t302;
    let t3206 = t3205 * t316;
    let t3207 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t3206;
    let t3208 = t1148 * t749;
    let t3210 = t1148 * t753;
    let t3212 = t3176 + t3179 + t3181 / F::cast_from(16.0_f64) - t3183 / F::cast_from(48.0_f64) + t3188 / F::cast_from(768.0_f64) + t3193 + t3196 / F::cast_from(192.0_f64) - t3198 / F::cast_from(1536.0_f64) - t3200 / F::cast_from(1536.0_f64) + t3203 + t3207 + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t3208 - t3210 / F::cast_from(384.0_f64);
    let t3213 = param_beta * t3212;
    let t3220 = t414 * t756;
    let t3221 = t3220 * t417;
    let t3225 = t1155 * t1163;
    let t3230 = t1884 * t123;
    let t3231 = t411 * t764;
    let t3232 = t3230 * t3231;
    let t3235 = t417 * t762;
    let t3236 = t415 * t3235;
    let t3238 = t331 * t295 * t121;
    let t3239 = t1165 * t3238;
    let t3242 = t1151 * t331;
    let t3243 = t1158 * t3242;
    let t3246 = t411 * t784;
    let t3247 = t1158 * t3246;
    let t3250 = t604 * t411;
    let t3251 = t3250 * t610;
    let t3254 = t282 * t1151;
    let t3255 = t3254 * t296;
    let t3258 = t1165 * t723;
    let t3260 = t1165 * t728;
    let t3262 = t419 * t3212;
    let t3264 = -F::cast_from(2.0_f64) * t1152 * t332 + F::cast_from(4.0_f64) * t1156 * t1160 - F::cast_from(2.0_f64) * t1156 * t1168 - F::cast_from(2.0_f64) * t1164 * t3251 + F::cast_from(2.0_f64) * t1164 * t3255 + t1164 * t3258 + t1164 * t3260 + F::cast_from(2.0_f64) * t3225 * t1166 + t3213 * t148 - t3221 * t420 - F::cast_from(6.0_f64) * t418 * t3232 - F::cast_from(4.0_f64) * t3236 * t3239 + F::cast_from(4.0_f64) * t418 * t3243 + F::cast_from(2.0_f64) * t418 * t3247 - t418 * t3262 + F::cast_from(2.0_f64) * t412 * t765 - t412 * t785;
    let t3268 = t1170 * t792;
    let t3269 = t3268 * t334;
    let t3272 = t422 * t1618;
    let t3273 = t3272 * t790;
    let t3276 = t1175 * t787;
    let tv3rho2sigma0 = t401 * t100 * t3264 * t335 - F::cast_from(6.0_f64) * t1128 * t1175 * t3166 + F::cast_from(6.0_f64) * t1128 * t3163 + F::cast_from(3.0_f64) * t1128 * t3170 - F::cast_from(2.0_f64) * t1174 * t3269 + F::cast_from(2.0_f64) * t1174 * t3273 - t1174 * t3276 + F::cast_from(6.0_f64) * t3159 * t3160 + F::cast_from(6.0_f64) * t3152 + F::cast_from(2.0_f64) * t3155 - F::cast_from(2.0_f64) * t3157;
    let t3278 = t252 * t1180;
    let t3280 = t1202 * t335;
    let t3281 = t101 * t3280;
    let t3283 = t101 * t1207;
    let t3285 = t1178 * t560;
    let t3288 = t3280 * t262;
    let t3294 = t1178 * t579;
    let t3297 = F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t3175;
    let t3306 = F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t3202;
    let t3310 = t3297 + F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t3178 + t3181 / F::cast_from(8.0_f64) - t3183 / F::cast_from(24.0_f64) + t3188 / F::cast_from(384.0_f64) + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t3192 + t3196 / F::cast_from(96.0_f64) - t3198 / F::cast_from(768.0_f64) - t3200 / F::cast_from(768.0_f64) + t3306 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t3206 + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t3208 - t3210 / F::cast_from(192.0_f64);
    let t3311 = param_beta * t3310;
    let t3326 = t3230 * t427 * t764;
    let t3329 = t1197 * t3238;
    let t3332 = t1188 * t331;
    let t3333 = t1158 * t3332;
    let t3337 = t1158 * t427 * t784;
    let t3340 = t604 * t427;
    let t3341 = t3340 * t610;
    let t3344 = t282 * t1188;
    let t3345 = t3344 * t296;
    let t3348 = t1197 * t723;
    let t3350 = t1197 * t728;
    let t3352 = t419 * t3310;
    let t3354 = F::cast_from(4.0_f64) * t1156 * t1194 - F::cast_from(2.0_f64) * t1156 * t1200 - F::cast_from(2.0_f64) * t1164 * t3341 + F::cast_from(2.0_f64) * t1164 * t3345 + t1164 * t3348 + t1164 * t3350 - F::cast_from(2.0_f64) * t1189 * t332 + F::cast_from(2.0_f64) * t3225 * t1198 + t3311 * t148 - t3221 * t430 - F::cast_from(4.0_f64) * t3236 * t3329 - F::cast_from(6.0_f64) * t418 * t3326 + F::cast_from(4.0_f64) * t418 * t3333 + F::cast_from(2.0_f64) * t418 * t3337 - t418 * t3352 + F::cast_from(2.0_f64) * t428 * t765 - t428 * t785;
    let t3358 = t1202 * t792;
    let t3359 = t3358 * t334;
    let t3362 = t432 * t1618;
    let t3363 = t3362 * t790;
    let t3366 = t1206 * t787;
    let tv3rho2sigma1 = t401 * t100 * t3354 * t335 - F::cast_from(6.0_f64) * t1128 * t1206 * t3166 + F::cast_from(6.0_f64) * t1128 * t3288 + F::cast_from(3.0_f64) * t1128 * t3294 - F::cast_from(2.0_f64) * t1174 * t3359 + F::cast_from(2.0_f64) * t1174 * t3363 - t1174 * t3366 + F::cast_from(6.0_f64) * t3159 * t3285 + F::cast_from(6.0_f64) * t3278 + F::cast_from(2.0_f64) * t3281 - F::cast_from(2.0_f64) * t3283;
    let tv3rho2sigma2 = tv3rho2sigma0;
    let t3369 = t252 * t1209;
    let t3374 = t3154 * t361;
    let t3377 = t361 * t334;
    let t3381 = t1126 * t831;
    let t3384 = t1233 * t335;
    let t3385 = t101 * t3384;
    let t3386 = t3384 * t262;
    let t3390 = t3177 * t366;
    let t3392 = t133 * t19;
    let t3393 = t402 * t3392;
    let t3394 = t111 * t919;
    let t3395 = t3393 * t3394;
    let t3397 = t1136 * t833;
    let t3400 = t280 * t3185 * t131;
    let t3401 = t3400 * t843;
    let t3403 = t3191 * t378;
    let t3405 = t3195 * t849;
    let t3407 = t1141 * t903;
    let t3409 = t3195 * t907;
    let t3413 = t3195 * t914;
    let t3415 = t3205 * t382;
    let t3417 = t1148 * t921;
    let t3419 = t1148 * t925;
    let t3421 = t3176 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t3178 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t3390 + t3395 / F::cast_from(16.0_f64) - t3397 / F::cast_from(48.0_f64) + t3401 / F::cast_from(768.0_f64) + F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t3403 + t3405 / F::cast_from(384.0_f64) - t3407 / F::cast_from(1536.0_f64) - t3409 / F::cast_from(1536.0_f64) + F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t3192 + t3203 + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t3206 + t3413 / F::cast_from(384.0_f64) + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t3415 + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t3417 - t3419 / F::cast_from(384.0_f64);
    let t3422 = param_beta * t3421;
    let t3426 = t412 * t116;
    let t3430 = t414 * t928;
    let t3431 = t3430 * t417;
    let t3435 = t1222 * t1163;
    let t3440 = t417 * t1884;
    let t3441 = t415 * t3440;
    let t3442 = t123 * t411;
    let t3443 = t393 * t331;
    let t3444 = t3442 * t3443;
    let t3448 = t393 * t295 * t121;
    let t3449 = t1165 * t3448;
    let t3452 = -t1152 * t394 + F::cast_from(2.0_f64) * t1156 * t1226 + F::cast_from(2.0_f64) * t1223 * t1160 + t3435 * t1166 - t1223 * t1168 - t1219 * t332 + t3422 * t148 - F::cast_from(2.0_f64) * t3236 * t3449 + F::cast_from(2.0_f64) * t3426 * t935 - t3431 * t420 - F::cast_from(6.0_f64) * t3441 * t3444 - t412 * t955;
    let t3453 = t1151 * t393;
    let t3454 = t1158 * t3453;
    let t3457 = t411 * t954;
    let t3458 = t1158 * t3457;
    let t3462 = t376 * t331;
    let t3463 = t1165 * t3462;
    let t3466 = t2303 * t295;
    let t3467 = t3250 * t3466;
    let t3470 = t3254 * t376;
    let t3472 = t1165 * t901;
    let t3474 = t2082 * t121;
    let t3475 = t1165 * t3474;
    let t3478 = t1218 * t331;
    let t3479 = t1158 * t3478;
    let t3482 = t282 * t1218;
    let t3483 = t3482 * t296;
    let t3485 = t419 * t3421;
    let t3487 = -t1156 * t1231 - F::cast_from(2.0_f64) * t1164 * t3467 + t1164 * t3470 + t1164 * t3472 + t1164 * t3475 + t1164 * t3483 + t3225 * t1229 - F::cast_from(2.0_f64) * t3236 * t3463 + F::cast_from(2.0_f64) * t418 * t3454 + F::cast_from(2.0_f64) * t418 * t3458 + F::cast_from(2.0_f64) * t418 * t3479 - t418 * t3485;
    let t3488 = t3452 + t3487;
    let t3492 = t1233 * t792;
    let t3493 = t3492 * t334;
    let t3495 = t101 * t1237;
    let t3496 = t396 * t262;
    let t3500 = t3268 * t396;
    let t3502 = t396 * t334;
    let t3506 = t1175 * t957;
    let tv3rho2sigma3 = t401 * t100 * t3488 * t335 + F::cast_from(6.0_f64) * t3159 * t1126 * t919 - F::cast_from(3.0_f64) * t1128 * t1175 * t3377 - F::cast_from(3.0_f64) * t1128 * t1175 * t3496 + F::cast_from(2.0_f64) * t1174 * t3272 * t3502 + F::cast_from(3.0_f64) * t1128 * t3374 + F::cast_from(3.0_f64) * t1128 * t3381 + F::cast_from(3.0_f64) * t1128 * t3386 - t1174 * t3493 - t1174 * t3500 - t1174 * t3506 + F::cast_from(3.0_f64) * t3152 + t3155 - t3157 + F::cast_from(3.0_f64) * t3369 + t3385 - t3495;
    let t3509 = t252 * t1239;
    let t3514 = t3280 * t361;
    let t3520 = t1178 * t831;
    let t3523 = t1258 * t335;
    let t3524 = t101 * t3523;
    let t3525 = t3523 * t262;
    let t3528 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t3390;
    let t3532 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t3403;
    let t3537 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t3415;
    let t3540 = t3297 + t3179 + t3528 + t3395 / F::cast_from(8.0_f64) - t3397 / F::cast_from(24.0_f64) + t3401 / F::cast_from(384.0_f64) + t3532 + t3405 / F::cast_from(192.0_f64) - t3407 / F::cast_from(768.0_f64) - t3409 / F::cast_from(768.0_f64) + t3193 + t3306 + t3207 + t3413 / F::cast_from(192.0_f64) + t3537 + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t3417 - t3419 / F::cast_from(192.0_f64);
    let t3541 = param_beta * t3540;
    let t3545 = t428 * t116;
    let t3556 = t123 * t427;
    let t3557 = t3556 * t3443;
    let t3560 = t1197 * t3448;
    let t3563 = F::cast_from(2.0_f64) * t1156 * t1251 - t1189 * t394 + F::cast_from(2.0_f64) * t1223 * t1194 + t3435 * t1198 - t1223 * t1200 - t1246 * t332 + t3541 * t148 - F::cast_from(2.0_f64) * t3236 * t3560 - t3431 * t430 - F::cast_from(6.0_f64) * t3441 * t3557 + F::cast_from(2.0_f64) * t3545 * t935 - t428 * t955;
    let t3564 = t1188 * t393;
    let t3565 = t1158 * t3564;
    let t3569 = t1158 * t427 * t954;
    let t3573 = t1197 * t3462;
    let t3576 = t3340 * t3466;
    let t3579 = t3344 * t376;
    let t3581 = t1197 * t901;
    let t3583 = t1197 * t3474;
    let t3586 = t1245 * t331;
    let t3587 = t1158 * t3586;
    let t3590 = t282 * t1245;
    let t3591 = t3590 * t296;
    let t3593 = t419 * t3540;
    let t3595 = -t1156 * t1256 - F::cast_from(2.0_f64) * t1164 * t3576 + t1164 * t3579 + t1164 * t3581 + t1164 * t3583 + t1164 * t3591 + t3225 * t1254 - F::cast_from(2.0_f64) * t3236 * t3573 + F::cast_from(2.0_f64) * t418 * t3565 + F::cast_from(2.0_f64) * t418 * t3569 + F::cast_from(2.0_f64) * t418 * t3587 - t418 * t3593;
    let t3596 = t3563 + t3595;
    let t3600 = t1258 * t792;
    let t3601 = t3600 * t334;
    let t3603 = t101 * t1262;
    let t3607 = t3358 * t396;
    let t3612 = t1206 * t957;
    let tv3rho2sigma4 = t401 * t100 * t3596 * t335 - F::cast_from(3.0_f64) * t1128 * t1206 * t3377 - F::cast_from(3.0_f64) * t1128 * t1206 * t3496 + F::cast_from(2.0_f64) * t1174 * t3362 * t3502 + F::cast_from(6.0_f64) * t3159 * t1178 * t919 + F::cast_from(3.0_f64) * t1128 * t3514 + F::cast_from(3.0_f64) * t1128 * t3520 + F::cast_from(3.0_f64) * t1128 * t3525 - t1174 * t3601 - t1174 * t3607 - t1174 * t3612 + F::cast_from(3.0_f64) * t3278 + t3281 - t3283 + F::cast_from(3.0_f64) * t3509 + t3524 - t3603;
    let tv3rho2sigma5 = tv3rho2sigma3;
    let t3617 = t1126 * t992;
    let t3620 = t3384 * t361;
    let t3623 = t361 * t396;
    let t3627 = t1126 * t1033;
    let t3630 = t3180 * t1039;
    let t3632 = t1136 * t1043;
    let t3634 = t3187 * t1049;
    let t3636 = t3195 * t1055;
    let t3638 = t1141 * t1076;
    let t3640 = t1141 * t1081;
    let t3642 = t1148 * t1086;
    let t3644 = t1148 * t1090;
    let t3646 = t3176 + t3528 + t3630 / F::cast_from(16.0_f64) - t3632 / F::cast_from(48.0_f64) + t3634 / F::cast_from(768.0_f64) + t3532 + t3636 / F::cast_from(192.0_f64) - t3638 / F::cast_from(1536.0_f64) - t3640 / F::cast_from(1536.0_f64) + t3203 + t3537 + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t3642 - t3644 / F::cast_from(384.0_f64);
    let t3647 = param_beta * t3646;
    let t3654 = t414 * t1093;
    let t3655 = t3654 * t417;
    let t3663 = t411 * t1098;
    let t3664 = t3230 * t3663;
    let t3668 = t393 * t375 * t121;
    let t3669 = t1165 * t3668;
    let t3672 = t1218 * t393;
    let t3673 = t1158 * t3672;
    let t3676 = t411 * t1117;
    let t3677 = t1158 * t3676;
    let t3680 = t3250 * t1047;
    let t3683 = t3482 * t376;
    let t3686 = t1165 * t1074;
    let t3688 = t1165 * t1079;
    let t3690 = t419 * t3646;
    let t3692 = F::cast_from(2.0_f64) * t412 * t1099 - t412 * t1118 - F::cast_from(2.0_f64) * t1164 * t3680 + F::cast_from(2.0_f64) * t1164 * t3683 + t1164 * t3686 + t1164 * t3688 - F::cast_from(2.0_f64) * t1219 * t394 + F::cast_from(4.0_f64) * t1223 * t1226 - F::cast_from(2.0_f64) * t1223 * t1231 + F::cast_from(2.0_f64) * t3435 * t1229 + t3647 * t148 - F::cast_from(4.0_f64) * t3236 * t3669 - t3655 * t420 - F::cast_from(6.0_f64) * t418 * t3664 + F::cast_from(4.0_f64) * t418 * t3673 + F::cast_from(2.0_f64) * t418 * t3677 - t418 * t3690;
    let t3696 = t3492 * t396;
    let t3699 = t3272 * t1017;
    let t3702 = t1175 * t1120;
    let tv3rho2sigma6 = t401 * t100 * t3692 * t335 - F::cast_from(6.0_f64) * t1128 * t1175 * t3623 + F::cast_from(6.0_f64) * t1128 * t3620 + F::cast_from(3.0_f64) * t1128 * t3627 - F::cast_from(2.0_f64) * t1174 * t3696 + F::cast_from(2.0_f64) * t1174 * t3699 - t1174 * t3702 + F::cast_from(6.0_f64) * t3159 * t3617 + F::cast_from(6.0_f64) * t3369 + F::cast_from(2.0_f64) * t3385 - F::cast_from(2.0_f64) * t3495;
    let t3707 = t1178 * t992;
    let t3710 = t3523 * t361;
    let t3716 = t1178 * t1033;
    let t3730 = t3297 + F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t3390 + t3630 / F::cast_from(8.0_f64) - t3632 / F::cast_from(24.0_f64) + t3634 / F::cast_from(384.0_f64) + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t3403 + t3636 / F::cast_from(96.0_f64) - t3638 / F::cast_from(768.0_f64) - t3640 / F::cast_from(768.0_f64) + t3306 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t3415 + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t3642 - t3644 / F::cast_from(192.0_f64);
    let t3731 = param_beta * t3730;
    let t3746 = t3230 * t427 * t1098;
    let t3749 = t1197 * t3668;
    let t3752 = t1245 * t393;
    let t3753 = t1158 * t3752;
    let t3757 = t1158 * t427 * t1117;
    let t3760 = t3340 * t1047;
    let t3763 = t3590 * t376;
    let t3766 = t1197 * t1074;
    let t3768 = t1197 * t1079;
    let t3770 = t419 * t3730;
    let t3772 = F::cast_from(2.0_f64) * t428 * t1099 - t428 * t1118 - F::cast_from(2.0_f64) * t1164 * t3760 + F::cast_from(2.0_f64) * t1164 * t3763 + t1164 * t3766 + t1164 * t3768 + F::cast_from(4.0_f64) * t1223 * t1251 - F::cast_from(2.0_f64) * t1223 * t1256 - F::cast_from(2.0_f64) * t1246 * t394 + F::cast_from(2.0_f64) * t3435 * t1254 + t3731 * t148 - F::cast_from(4.0_f64) * t3236 * t3749 - t3655 * t430 - F::cast_from(6.0_f64) * t418 * t3746 + F::cast_from(4.0_f64) * t418 * t3753 + F::cast_from(2.0_f64) * t418 * t3757 - t418 * t3770;
    let t3776 = t3600 * t396;
    let t3779 = t3362 * t1017;
    let t3782 = t1206 * t1120;
    let tv3rho2sigma7 = t401 * t100 * t3772 * t335 - F::cast_from(6.0_f64) * t1128 * t1206 * t3623 + F::cast_from(6.0_f64) * t1128 * t3710 + F::cast_from(3.0_f64) * t1128 * t3716 - F::cast_from(2.0_f64) * t1174 * t3776 + F::cast_from(2.0_f64) * t1174 * t3779 - t1174 * t3782 + F::cast_from(6.0_f64) * t3159 * t3707 + F::cast_from(6.0_f64) * t3509 + F::cast_from(2.0_f64) * t3524 - F::cast_from(2.0_f64) * t3603;
    let tv3rho2sigma8 = tv3rho2sigma6;
    let t3784 = t1291 * t335;
    let t3785 = t101 * t3784;
    let t3786 = t3784 * t262;
    let t3789 = t1264 * t417;
    let t3790 = t282 * t129;
    let t3791 = t3790 * t130;
    let t3792 = t3789 * t3791;
    let t3793 = t6 * t147;
    let t3795 = t839 * t3793 * t296;
    let t3796 = t3792 * t3795;
    let t3800 = t1264 * t1265 * t302 * t1269;
    let t3801 = F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t3800;
    let t3804 = t3789 * t123 * t129 * t130;
    let t3806 = t622 * t3793 * t262;
    let t3807 = t3804 * t3806;
    let t3809 = t6 * t323;
    let t3811 = t839 * t3809 * t331;
    let t3812 = t3804 * t3811;
    let t3814 = t414 * t411;
    let t3815 = t3814 * t417;
    let t3818 = t1273 * t417;
    let t3819 = t1158 * t331;
    let t3822 = t1286 * t296;
    let t3825 = t1277 * t319;
    let t3826 = t3825 * t1280;
    let t3829 = t1884 * t282;
    let t3830 = t1272 * t331;
    let t3831 = t3829 * t3830;
    let t3834 = t1280 * t762;
    let t3835 = t1278 * t3834;
    let t3836 = t604 * t1272;
    let t3837 = t3836 * t296;
    let t3840 = t411 * t1151;
    let t3841 = t1282 * t3840;
    let t3844 = t3826 * t1287;
    let t3845 = t3844 * t138;
    let t3847 = t1282 * param_BB;
    let t3848 = t1281 * t3847;
    let t3849 = t6 * t331;
    let t3850 = t136 * t3849;
    let t3851 = t134 * t3850;
    let t3852 = t3848 * t3851;
    let t3854 = t323 * t604;
    let t3855 = param_BB * t129;
    let t3856 = t3854 * t3855;
    let t3857 = t1281 * t3856;
    let t3858 = t3857 * t298;
    let t3860 = t1288 * t305;
    Chunk2Out::<F> { t2196: t2196, t2202: t2202, t2204: t2204, t2206: t2206, t2208: t2208, t2215: t2215, t2217: t2217, t2218: t2218, t2221: t2221, t2223: t2223, t2224: t2224, t2229: t2229, t2244: t2244, t2247: t2247, t2248: t2248, t2251: t2251, t2254: t2254, t2257: t2257, t2258: t2258, t2260: t2260, t2265: t2265, t2267: t2267, t2270: t2270, t2272: t2272, t2279: t2279, t2280: t2280, t2292: t2292, t2296: t2296, t2300: t2300, t2303: t2303, t2304: t2304, t2306: t2306, t2316: t2316, t2317: t2317, t2325: t2325, t2330: t2330, t2331: t2331, t2334: t2334, t2335: t2335, t2338: t2338, t2356: t2356, t2359: t2359, t2366: t2366, t2384: t2384, t2391: t2391, t2392: t2392, t2394: t2394, t2395: t2395, t2396: t2396, t2397: t2397, t2399: t2399, t2402: t2402, t2405: t2405, t2406: t2406, t2409: t2409, t2411: t2411, t2412: t2412, t2413: t2413, t2414: t2414, t2417: t2417, t2420: t2420, t2424: t2424, t2425: t2425, t2426: t2426, t2429: t2429, t2432: t2432, t2434: t2434, t2435: t2435, t2438: t2438, t2440: t2440, t2442: t2442, t2443: t2443, t2446: t2446, t2460: t2460, t2461: t2461, t2464: t2464, t2472: t2472, t2473: t2473, t2476: t2476, t2477: t2477, t2480: t2480, t2483: t2483, t2485: t2485, t2486: t2486, t2489: t2489, t2491: t2491, t2498: t2498, t2500: t2500, t2503: t2503, t2506: t2506, t2508: t2508, t2509: t2509, t2510: t2510, t2512: t2512, t2513: t2513, t2515: t2515, t2517: t2517, t2521: t2521, t2526: t2526, t2531: t2531, t2536: t2536, t2541: t2541, t2544: t2544, t2550: t2550, t2551: t2551, t2552: t2552, t2553: t2553, t2554: t2554, t2555: t2555, t2560: t2560, t2567: t2567, t2572: t2572, t2580: t2580, t2581: t2581, t2582: t2582, t2585: t2585, t2587: t2587, t2591: t2591, t2593: t2593, t2596: t2596, t2598: t2598, t2601: t2601, t2603: t2603, t2613: t2613, t2619: t2619, t2631: t2631, t2632: t2632, t2635: t2635, t2640: t2640, t2641: t2641, t2644: t2644, t2647: t2647, t2648: t2648, t2650: t2650, t2654: t2654, t2657: t2657, t2659: t2659, t2669: t2669, t2670: t2670, t2678: t2678, t2682: t2682, t2686: t2686, t2690: t2690, t2697: t2697, t2701: t2701, t2705: t2705, t2709: t2709, t2711: t2711, t2715: t2715, t2720: t2720, t2722: t2722, t2725: t2725, t2727: t2727, t2730: t2730, t2732: t2732, t2737: t2737, t2741: t2741, t2745: t2745, t2749: t2749, t2752: t2752, t2754: t2754, t2758: t2758, t2761: t2761, t2763: t2763, t2774: t2774, t2775: t2775, t2780: t2780, t2787: t2787, t2788: t2788, t2791: t2791, t2795: t2795, t2796: t2796, t2802: t2802, t2815: t2815, t2819: t2819, t2829: t2829, t2832: t2832, t2849: t2849, t2850: t2850, t2852: t2852, t2853: t2853, t2854: t2854, t2855: t2855, t2856: t2856, t2859: t2859, t2870: t2870, t2871: t2871, t2872: t2872, t2878: t2878, t2883: t2883, t2888: t2888, t2894: t2894, t2895: t2895, t2896: t2896, t2898: t2898, t2899: t2899, t2900: t2900, t2901: t2901, t2902: t2902, t2904: t2904, t2905: t2905, t2907: t2907, t2908: t2908, t2912: t2912, t2913: t2913, t2914: t2914, t2915: t2915, t2916: t2916, t2917: t2917, t2920: t2920, t2921: t2921, t2922: t2922, t2923: t2923, t2924: t2924, t2925: t2925, t2926: t2926, t2928: t2928, t2929: t2929, t2930: t2930, t2931: t2931, t2932: t2932, t2933: t2933, t2935: t2935, t2937: t2937, t2938: t2938, t2941: t2941, t2959: t2959, t2960: t2960, t2961: t2961, t2962: t2962, t2964: t2964, t2967: t2967, t2968: t2968, t2970: t2970, t2973: t2973, t2975: t2975, t2980: t2980, t2986: t2986, t2988: t2988, t2995: t2995, t2999: t2999, t3007: t3007, t3010: t3010, t3013: t3013, t3016: t3016, t3017: t3017, t3019: t3019, t3022: t3022, t3024: t3024, t3030: t3030, t3034: t3034, t3036: t3036, t3040: t3040, t3043: t3043, t3045: t3045, t3049: t3049, t3052: t3052, t3054: t3054, t3057: t3057, t3059: t3059, t3063: t3063, t3066: t3066, t3067: t3067, t3075: t3075, t3076: t3076, t3080: t3080, t3081: t3081, t3089: t3089, t3090: t3090, t3093: t3093, t3096: t3096, t3100: t3100, t3110: t3110, t3114: t3114, t3121: t3121, t3131: t3131, t3132: t3132, t3134: t3134, t3135: t3135, t3136: t3136, t3154: t3154, t3159: t3159, t3160: t3160, t3163: t3163, t3166: t3166, t3170: t3170, t3173: t3173, t3177: t3177, t3185: t3185, t3187: t3187, t3191: t3191, t3195: t3195, t3205: t3205, t3212: t3212, t3213: t3213, t3220: t3220, t3221: t3221, t3225: t3225, t3230: t3230, t3231: t3231, t3232: t3232, t3235: t3235, t3236: t3236, t3238: t3238, t3239: t3239, t3242: t3242, t3243: t3243, t3246: t3246, t3247: t3247, t3250: t3250, t3251: t3251, t3254: t3254, t3255: t3255, t3258: t3258, t3260: t3260, t3262: t3262, t3264: t3264, t3268: t3268, t3269: t3269, t3272: t3272, t3273: t3273, t3276: t3276, t3280: t3280, t3285: t3285, t3288: t3288, t3294: t3294, t3310: t3310, t3311: t3311, t3326: t3326, t3329: t3329, t3332: t3332, t3333: t3333, t3337: t3337, t3340: t3340, t3341: t3341, t3344: t3344, t3345: t3345, t3348: t3348, t3350: t3350, t3352: t3352, t3354: t3354, t3358: t3358, t3359: t3359, t3362: t3362, t3363: t3363, t3366: t3366, t3374: t3374, t3377: t3377, t3381: t3381, t3384: t3384, t3386: t3386, t3392: t3392, t3393: t3393, t3394: t3394, t3400: t3400, t3421: t3421, t3422: t3422, t3426: t3426, t3430: t3430, t3431: t3431, t3435: t3435, t3440: t3440, t3441: t3441, t3442: t3442, t3443: t3443, t3444: t3444, t3448: t3448, t3449: t3449, t3453: t3453, t3454: t3454, t3457: t3457, t3458: t3458, t3462: t3462, t3463: t3463, t3466: t3466, t3467: t3467, t3470: t3470, t3472: t3472, t3474: t3474, t3475: t3475, t3478: t3478, t3479: t3479, t3482: t3482, t3483: t3483, t3485: t3485, t3488: t3488, t3492: t3492, t3493: t3493, t3496: t3496, t3500: t3500, t3502: t3502, t3506: t3506, t3514: t3514, t3520: t3520, t3523: t3523, t3525: t3525, t3540: t3540, t3541: t3541, t3545: t3545, t3556: t3556, t3557: t3557, t3560: t3560, t3564: t3564, t3565: t3565, t3569: t3569, t3573: t3573, t3576: t3576, t3579: t3579, t3581: t3581, t3583: t3583, t3586: t3586, t3587: t3587, t3590: t3590, t3591: t3591, t3593: t3593, t3596: t3596, t3600: t3600, t3601: t3601, t3607: t3607, t3612: t3612, t3617: t3617, t3620: t3620, t3623: t3623, t3627: t3627, t3646: t3646, t3647: t3647, t3654: t3654, t3655: t3655, t3663: t3663, t3664: t3664, t3668: t3668, t3669: t3669, t3672: t3672, t3673: t3673, t3676: t3676, t3677: t3677, t3680: t3680, t3683: t3683, t3686: t3686, t3688: t3688, t3690: t3690, t3692: t3692, t3696: t3696, t3699: t3699, t3702: t3702, t3707: t3707, t3710: t3710, t3716: t3716, t3730: t3730, t3731: t3731, t3746: t3746, t3749: t3749, t3752: t3752, t3753: t3753, t3757: t3757, t3760: t3760, t3763: t3763, t3766: t3766, t3768: t3768, t3770: t3770, t3772: t3772, t3776: t3776, t3779: t3779, t3782: t3782, t3784: t3784, t3785: t3785, t3786: t3786, t3789: t3789, t3790: t3790, t3791: t3791, t3792: t3792, t3793: t3793, t3795: t3795, t3796: t3796, t3800: t3800, t3801: t3801, t3804: t3804, t3806: t3806, t3807: t3807, t3809: t3809, t3811: t3811, t3812: t3812, t3814: t3814, t3815: t3815, t3818: t3818, t3819: t3819, t3822: t3822, t3825: t3825, t3826: t3826, t3829: t3829, t3830: t3830, t3831: t3831, t3834: t3834, t3835: t3835, t3836: t3836, t3837: t3837, t3840: t3840, t3841: t3841, t3844: t3844, t3845: t3845, t3847: t3847, t3848: t3848, t3849: t3849, t3850: t3850, t3851: t3851, t3852: t3852, t3854: t3854, t3855: t3855, t3856: t3856, t3857: t3857, t3858: t3858, t3860: t3860, tv3rho31: tv3rho31, tv3rho32: tv3rho32, tv3rho33: tv3rho33, tv3rho2sigma0: tv3rho2sigma0, tv3rho2sigma1: tv3rho2sigma1, tv3rho2sigma2: tv3rho2sigma2, tv3rho2sigma3: tv3rho2sigma3, tv3rho2sigma4: tv3rho2sigma4, tv3rho2sigma5: tv3rho2sigma5, tv3rho2sigma6: tv3rho2sigma6, tv3rho2sigma7: tv3rho2sigma7, tv3rho2sigma8: tv3rho2sigma8 }
}
